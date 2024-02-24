//! The angreal `init` command.
//!
use crate::{
    git::{git_clone, git_pull_ff, remote_exists},
    utils::{context_to_map, render_dir, repl_context_from_toml},
};

use git_url_parse::{GitUrl, Scheme};
use home::home_dir;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};

use std::{
    env, fs,
    fs::File,
    io::Write,
    ops::Not,
    path::{Path, PathBuf},
    process::exit,
};
use toml::Value;

use log::{debug, error};

/// Initialize a new project by rendering a template.
pub fn init(template: &str, force: bool, take_inputs: bool) {
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

    let rendered_dot_angreal_path = render_template(Path::new(&template), take_inputs, force);

    let mut rendered_angreal_init = Path::new(&rendered_dot_angreal_path).to_path_buf();
    rendered_angreal_init.push("init.py");

    if rendered_angreal_init.is_file() {
        let init_contents = fs::read_to_string(rendered_angreal_init).unwrap();
        // Get our init function
        Python::with_gil(|py| {
            let syspath: &PyList = py
                .import("sys")
                .unwrap()
                .getattr("path")
                .unwrap()
                .downcast::<PyList>()
                .unwrap();
            syspath
                .insert(0, rendered_dot_angreal_path.clone())
                .unwrap();

            let function: Py<PyAny> = PyModule::from_code(py, &init_contents, "", "")
                .unwrap()
                .getattr("init")
                .unwrap()
                .into();

            function.call0(py).unwrap();
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

fn handle_file_template(template: &str, angreal_home: &PathBuf) -> String {
    let mut try_template = angreal_home.clone();
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
        let mut try_supported = angreal_home.clone();
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
                let mut dst = angreal_home.clone();
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
        .unwrap_or(Path::new(&remote.path))
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
fn create_home_dot_angreal() -> PathBuf {
    let mut home_dir = home_dir().unwrap();
    home_dir.push(".angrealrc");

    if home_dir.exists().not() {
        fs::create_dir(&home_dir).unwrap();
    }
    debug!("Angreal home directory location is {:?}", home_dir);
    home_dir
}

/// render the provided angreal template path
fn render_template(path: &Path, take_input: bool, force: bool) -> String {
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

    // create a tera context from the toml file interactively.
    let context = repl_context_from_toml(toml, take_input);
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

#[cfg(test)]
#[path = "../tests"]
mod tests {

    use std::ops::Not;
    use std::path::{Path, PathBuf};
    use std::{env, fs};
    use tempfile::tempdir;

    #[test]
    fn test_init_from_git() {
        crate::init::init(
            "https://github.com/angreal/angreal_test_template.git",
            true,
            false,
        );
        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("angreal_test_project"));
        let _ = fs::remove_dir_all(&rendered_root);

        let _ = fs::remove_dir_all(crate::init::create_home_dot_angreal());
    }

    #[test]
    fn test_handle_file_template() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().expect("Failed to create temporary directory");

        // Create a dummy template directory within the temporary directory
        let template_dir = temp_dir.path().join("template");
        std::fs::create_dir(&template_dir).expect("Failed to create template directory");

        // Test case 1: Template directory exists with angreal.toml
        let angreal_toml = template_dir.join("angreal.toml");
        std::fs::File::create(&angreal_toml).expect("Failed to create angreal.toml");
        assert_eq!(
            crate::init::handle_file_template(
                template_dir.to_str().unwrap(),
                &temp_dir.path().to_path_buf()
            ),
            template_dir.to_str().unwrap().to_string()
        );

        // Test case 2: Template directory exists without angreal.toml
        std::fs::remove_file(&angreal_toml).expect("Failed to remove angreal.toml");
        assert_eq!(
            crate::init::handle_file_template(
                template_dir.to_str().unwrap(),
                &temp_dir.path().to_path_buf()
            ),
            template_dir.to_str().unwrap().to_string()
        );

        // Clean up
        temp_dir
            .close()
            .expect("Failed to clean up temporary directory");
    }

    #[test]
    fn test_handle_git_template() {
        // Setup
        let temp_dir = tempdir().unwrap();
        let target_dir = temp_dir.path().to_path_buf();
        let test_git_url = "https://github.com/angreal/angreal_test_template.git"; // Example URL

        crate::init::handle_git_template(test_git_url, target_dir.clone());

        let expected_path = target_dir.join("angreal").join("angreal_test_template");
        assert!(expected_path.exists());
    }

    #[test]
    fn test_init_long() {
        // clone
        crate::init::init(
            "https://github.com/angreal/angreal_test_template.git",
            true,
            false,
        );
        // clean up rendered
        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("angreal_test_project"));
        let _ = fs::remove_dir_all(&rendered_root);
        // use the long version
        crate::init::init("angreal/angreal_test_template", true, false);
        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("angreal_test_project"));
        let _ = fs::remove_dir_all(&rendered_root);
        let _ = fs::remove_dir_all(crate::init::create_home_dot_angreal());
    }

    #[test]
    fn test_init_short() {
        // clone
        crate::init::init("angreal_test_template", true, false);
        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("angreal_test_project"));
        let _ = fs::remove_dir_all(&rendered_root);
        let _ = fs::remove_dir_all(crate::init::create_home_dot_angreal());
    }

    #[test]
    fn test_render_template() {
        let mut template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        template_root.push(Path::new("tests/common/test_assets/test_template"));
        crate::init::render_template(&template_root, false, true);

        let mut angreal_toml = template_root.clone();
        angreal_toml.push("angreal.toml");

        let mut assets = template_root.clone();
        assets.push("assets");

        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("folder_name"));

        let mut dot_angreal = rendered_root.clone();
        dot_angreal.push(Path::new(".angreal"));

        let mut readme_rst = rendered_root.clone();
        readme_rst.push("README.rst");

        let assets_no_exists = assets.is_dir().not();
        let dot_angreal_exists = dot_angreal.is_dir();
        let readme_rst_exists = readme_rst.is_file();
        let rendered_root_exists = rendered_root.is_dir();

        fs::remove_dir_all(&rendered_root).unwrap_or(());

        assert!(assets_no_exists);
        assert!(rendered_root_exists);
        assert!(dot_angreal_exists);
        assert!(readme_rst_exists);
    }

    #[test]
    fn test_home_dot_angreal() {
        crate::init::create_home_dot_angreal();
    }

    #[test]
    fn test_get_schema() {
        let url_https = "https://github.com/angreal/angreal_test_template.git";
        let url_ssh = "git@github.com:angreal/angreal_test_template.git";
        let url_git = "git:github.com/angreal/angreal_test_template.git";
        let url_file = "path/angreal/angreal.git";
        let url_dir = "tests/common/test_assets/";
        let str_str = "python3";

        let https_schema = crate::init::get_scheme(url_https);
        assert_eq!(https_schema.unwrap(), "https");

        let ssh_schema = crate::init::get_scheme(url_ssh);
        assert_eq!(ssh_schema.unwrap(), "ssh");

        let git_schema = crate::init::get_scheme(url_git);
        assert_eq!(git_schema.unwrap(), "git");

        let file_schema = crate::init::get_scheme(url_file);
        assert_eq!(file_schema.unwrap(), "file");

        let local_dir = crate::init::get_scheme(url_dir);
        assert_eq!(local_dir.unwrap(), "file");

        let str_schema = crate::init::get_scheme(str_str);
        assert_eq!(str_schema.unwrap(), "file");
    }
}
