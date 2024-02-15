//! The angreal `init` command.
//!
use crate::git::{git_clone, git_pull_ff, remote_exists};
use crate::utils::{context_to_map, render_dir, repl_context_from_toml};

use git_url_parse::{GitUrl, Scheme};
use home::home_dir;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::exit;
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
            let remote = GitUrl::parse(template).unwrap();
            let mut dst = angreal_home;

            let mut path = Path::new(&GitUrl::parse(&remote.to_string()).unwrap().path)
                .to_path_buf()
                .with_extension("");

            // strip leading slash if needed
            if path.starts_with("/") {
                path = path.strip_prefix("/").unwrap().to_path_buf();
            }
            dst.push(path.to_str().unwrap());

            if dst.is_dir() {
                debug!("Template exists at {:?}, attempting ff-pull.", dst);
                git_pull_ff(dst.to_str().unwrap())
            } else {
                debug!("Template does not exist at {:?}, attempting clone", dst);
                git_clone(template, dst.to_str().unwrap())
            }
        }
        "file" => {
            // if someone runs `angreal init org/template`, we check ~/.angrealrc/org/template
            // for the template to use and attempt a ff-pull on that repo
            let mut try_template = angreal_home.clone();
            try_template.push(Path::new(template));

            //  First we try ~/.angrealrc/template for a template with that name
            if try_template.is_dir() {
                let mut git_location = try_template.clone();
                git_location.push(Path::new(".git"));

                if git_location.exists() {
                    // Only attempt a ff-pull if it is a git repo
                    debug!("Template exists at {:?}, attempting ff-pull.", try_template);
                    git_pull_ff(try_template.to_str().unwrap())
                } else {
                    debug!("Bare template found at {:?}, using.", try_template);
                    try_template
                }
            } else if Path::new(template).is_dir() {
                // then we see if it's just a local angreal template

                let mut angreal_toml = Path::new(template).to_path_buf();
                angreal_toml.push("angreal.toml");

                if angreal_toml.is_file() {
                    debug!(
                        "Directory exists at {:?}, checking for angreal.toml at {:?}",
                        try_template, angreal_toml
                    );
                    Path::new(template).to_path_buf()
                } else {
                    error!(
                        "The template {}, doesn't appear to exist locally at {}",
                        template, template
                    );
                    exit(1);
                }
            } else {
                // if someone enters angreal init python , it should try check ~/.angrealrc/angreal/python
                let mut try_supported = angreal_home.clone();
                try_supported.push("angreal");
                try_supported.push(Path::new(template));

                if try_supported.is_dir() {
                    let mut git_location = try_supported.clone();
                    git_location.push(Path::new(".git"));

                    if git_location.exists() {
                        // Only attempt a ff-pull if it is a git repo
                        debug!("Template exists at {:?}, attempting ff-pull.", try_template);
                        git_pull_ff(try_supported.to_str().unwrap())
                    } else {
                        error!(
                            "The template {}, doesn't appear to exist locally at {}",
                            template,
                            try_supported.display()
                        );
                        exit(1);
                    }
                } else {
                    // if that doesn't work we should attempt to clone from github.com/angreal/python
                    let maybe_repo = format!("https://github.com/angreal/{template}.git");
                    debug!(
                        "Template does not exist at {:?}, attempting clone",
                        &maybe_repo
                    );
                    if remote_exists(&maybe_repo) {
                        let mut dst = angreal_home;

                        let mut path = Path::new(&GitUrl::parse(maybe_repo.as_str()).unwrap().path)
                            .to_path_buf()
                            .with_extension("");

                        if path.starts_with("/") {
                            path = path.strip_prefix("/").unwrap().to_path_buf();
                        }
                        dst.push(path.to_str().unwrap());

                        git_clone(&maybe_repo, dst.to_str().unwrap())
                    } else {
                        // if that doesn't work we should fail
                        error!(
                            "The template {}, doesn't appear to exist locally or remotely.",
                            template
                        );
                        exit(1);
                    }
                }
            }
        }
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
fn get_scheme(u: &str) -> Result<String, ()> {
    let s = GitUrl::parse(u).unwrap();

    match s.scheme {
        Scheme::Https => Ok("https".to_string()),
        Scheme::GitSsh => Ok("gitssh".to_string()),
        Scheme::Ssh => Ok("ssh".to_string()),
        Scheme::Git => Ok("git".to_string()),
        Scheme::File => Ok("file".to_string()),
        _ => Err(()),
    }
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
