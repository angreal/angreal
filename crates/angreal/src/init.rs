//! The angreal `init` command.
//!
use crate::{
    git::{git_clone, git_pull_ff, remote_exists},
    utils::{context_to_map, render_dir, repl_context_from_toml},
};

use git_url_parse::{GitUrl, Scheme};
use home::home_dir;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use std::{
    env,
    fs::{self, File},
    io::Write,
    ops::Not,
    path::{Path, PathBuf},
    process::exit,
};
use toml::Value;

use log::{debug, error};

/// Initialize a new project by rendering a template.
pub fn init(template: &str, force: bool, take_inputs: bool, values_file: Option<&str>) {
    let angreal_home = create_home_dot_angreal();
    let template_type = get_scheme(template).unwrap();

    debug!("Got template type {:?} for {:?}.", template_type, template);

    debug!("Template is of type {:?}", template_type.as_str());
    let template = match template_type.as_str() {
        "https" | "gitssh" | "ssh" | "git" => {
            // If we get a git url , go get it either by a clone if it doesn't
            // already exist, or as a ff pull if it does
            handle_git_template(template, angreal_home)
        }
        "file" => PathBuf::from(handle_file_template(template, &angreal_home)),
        &_ => {
            error!(
                "Unhandled template type {} from {}, exiting.",
                template_type.as_str(),
                template
            );
            exit(1);
        }
    };

    let rendered_dot_angreal_path =
        render_template(Path::new(&template), take_inputs, force, values_file);

    let mut rendered_angreal_init = Path::new(&rendered_dot_angreal_path).to_path_buf();
    rendered_angreal_init.push("init.py");

    if rendered_angreal_init.is_file() {
        let init_contents = fs::read_to_string(rendered_angreal_init).unwrap();
        // Get our init function
        Python::attach(|py| {
            // Change to the rendered directory before executing Python code
            let current_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&rendered_dot_angreal_path) {
                error!("Failed to change to rendered directory: {}", e);
                exit(1);
            }

            use std::ffi::CString;
            let init_cstr = CString::new(init_contents).unwrap();
            let function: Py<PyAny> = PyModule::from_code(py, init_cstr.as_c_str(), c"", c"")
                .unwrap()
                .getattr("init")
                .unwrap()
                .unbind();

            match function.call0(py) {
                Ok(_) => debug!("Successfully executed init.py"),
                Err(err) => {
                    use crate::error_formatter::PythonErrorFormatter;
                    error!("Failed to execute init.py");
                    let formatter = PythonErrorFormatter::new(err);
                    println!("{}", formatter);
                    // Change back to original directory before exiting
                    let _ = env::set_current_dir(current_dir);
                    std::process::exit(1);
                }
            };

            // Change back to original directory after successful execution
            if let Err(e) = env::set_current_dir(current_dir) {
                error!("Failed to change back to original directory: {}", e);
                exit(1);
            }
        });
    }

    println!(
        "Angreal template ({}) successfully rendered !",
        template.to_string_lossy()
    );
}

/// get the schema for the provided template
fn get_scheme(u: &str) -> Result<String, String> {
    let s = GitUrl::parse(u).map_err(|_| format!("Failed to parse URL: {u}"))?;

    match s.scheme {
        Scheme::Https => Ok("https".to_string()),
        Scheme::GitSsh => Ok("gitssh".to_string()),
        Scheme::Ssh => Ok("ssh".to_string()),
        Scheme::Git => Ok("git".to_string()),
        Scheme::File => Ok("file".to_string()),
        _ => Err("Unsupported URL scheme".to_string()),
    }
}

fn handle_file_template(template: &str, angreal_home: &Path) -> String {
    let mut try_template = angreal_home.to_path_buf();
    try_template.push(Path::new(template));

    if try_template.is_dir() {
        let mut git_location = try_template.clone();
        git_location.push(Path::new(".git"));

        if git_location.exists() {
            debug!("Template exists at {:?}, attempting ff-pull.", try_template);
            git_pull_ff(try_template.to_str().unwrap())
                .to_string_lossy()
                .to_string()
        } else {
            debug!("Bare template found at {:?}, using.", try_template);
            try_template.to_string_lossy().to_string()
        }
    } else if Path::new(template).is_dir() {
        let mut angreal_toml = Path::new(template).to_path_buf();
        angreal_toml.push("angreal.toml");

        if angreal_toml.is_file() {
            debug!(
                "Directory exists at {:?}, checking for angreal.toml at {:?}",
                try_template, angreal_toml
            );
            Path::new(template)
                .to_path_buf()
                .to_string_lossy()
                .to_string()
        } else {
            error!(
                "The template {}, doesn't appear to exist locally at {}",
                template, template
            );
            exit(1);
        }
    } else {
        let mut try_supported = angreal_home.to_path_buf();
        try_supported.push("angreal");
        try_supported.push(Path::new(template));

        if try_supported.is_dir() {
            let mut git_location = try_supported.clone();
            git_location.push(Path::new(".git"));

            if git_location.exists() {
                debug!("Template exists at {:?}, attempting ff-pull.", try_template);
                git_pull_ff(try_supported.to_str().unwrap())
                    .to_string_lossy()
                    .to_string()
            } else {
                error!(
                    "The template {}, doesn't appear to exist locally at {}",
                    template,
                    try_supported.display()
                );
                exit(1);
            }
        } else {
            let maybe_repo = format!("https://github.com/angreal/{template}.git");
            debug!(
                "Template does not exist at {:?}, attempting clone",
                &maybe_repo
            );
            if remote_exists(&maybe_repo) {
                let mut dst = angreal_home.to_path_buf();
                let mut path = Path::new(
                    &GitUrl::parse(maybe_repo.as_str())
                        .expect("Failed to parse Git URL")
                        .path,
                )
                .to_path_buf()
                .with_extension("");

                if path.starts_with("/") {
                    path = path.strip_prefix("/").unwrap().to_path_buf();
                }
                dst.push(path.to_str().unwrap());

                git_clone(&maybe_repo, dst.to_str().unwrap())
                    .to_string_lossy()
                    .to_string()
            } else {
                error!(
                    "The template {}, doesn't appear to exist locally or remotely.",
                    template
                );
                exit(1);
            }
        }
    }
}

fn handle_git_template(template: &str, angreal_home: PathBuf) -> PathBuf {
    let remote = GitUrl::parse(template).expect("Failed to parse Git URL");
    // Compute destination path with the necessary adjustments
    let path = Path::new(&remote.path)
        .strip_prefix("/")
        .unwrap_or_else(|_| Path::new(&remote.path))
        .with_extension("");
    let dst = angreal_home.join(path);

    if dst.exists() {
        debug!("Template exists, attempting ff-pull at {:?}", dst);
        git_pull_ff(dst.to_str().unwrap());
    } else {
        debug!("Template does not exist, attempting clone to {:?}", dst);
        git_clone(template, dst.to_str().unwrap());
    }

    dst
}

/// create the angreal caching directory for storing cloned templates
pub fn create_home_dot_angreal() -> PathBuf {
    let mut home_dir = home_dir().unwrap();
    home_dir.push(".angrealrc");

    if home_dir.exists().not() {
        fs::create_dir(&home_dir).unwrap();
    }
    debug!("Angreal home directory location is {:?}", home_dir);
    home_dir
}

/// render the provided angreal template path
pub fn render_template(
    path: &Path,
    take_input: bool,
    force: bool,
    values_file: Option<&str>,
) -> String {
    // Verify the provided template path is minimially compliant.
    let mut toml = path.to_path_buf();
    toml.push(Path::new("angreal.toml"));
    debug!("angreal.toml should be at {:?}", toml);
    if toml.is_file().not() {
        error!(
            "`angreal.toml` not found where expected {:}",
            toml.display()
        );
    }

    // This is a replacement for a defensive check and is the closest thing to a ternary I've seen so far.
    // Evaluates to : if values file is None, do the first closure, other wise do the second closure
    let context = values_file.map_or_else(
        || repl_context_from_toml(toml.to_path_buf(), take_input),
        |file| repl_context_from_toml(Path::new(&file).to_path_buf(), false),
    );

    // create a tera context from the toml file interactively.

    let ctx = context.clone();

    // render the provided template directory
    let rendered_files = render_dir(path, context, &env::current_dir().unwrap(), force);

    let toml_values = context_to_map(ctx);
    let toml_string = toml::to_string(&Value::Table(toml_values)).unwrap();

    for f in rendered_files {
        if f.ends_with(".angreal") {
            let mut value_path = PathBuf::new();
            value_path.push(f.as_str());
            value_path.push("angreal.toml");
            let mut output = File::create(&value_path).unwrap();
            write!(output, "{}", toml_string.as_str()).unwrap();
            debug!("Storing initialization values to {}", &value_path.display());
            return f;
        }
    }

    // let mut output = File::create(&value_path).unwrap();
    // write!(output, "{}", toml_string.as_str()).unwrap();
    // debug!("Storing initialization values to {}", &value_path.display());
    // angreal_path
    String::new()
    // return path to .angreal
}
