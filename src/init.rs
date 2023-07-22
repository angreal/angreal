//! The angreal `init` command.
//!
use crate::git::{git_clone, git_pull_ff};

use git_url_parse::{GitUrl, Scheme};
use home::home_dir;

use glob::glob;
use log::{debug, error};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::exit;
use tera::{Context, Tera};
use text_io::read;
use toml::{map::Map, Table, Value};
use walkdir::WalkDir;

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
            dst.push(remote.name.as_str());

            if dst.is_dir() {
                debug!("Template exists at {:?}, attempting ff-pull.", dst);
                git_pull_ff(dst.to_str().unwrap())
            } else {
                debug!("Template does not exist at {:?}, attempting clone", dst);
                git_clone(template, dst.to_str().unwrap())
            }
        }
        "file" => {
            // if someone runs `angreal init template`, we check ~/.angreal/template
            // for the template to use and attempt a ff-pull on that repo
            let mut try_template = angreal_home;
            try_template.push(Path::new(template));

            //  First we try ~/.angreal for a template with that name
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
                    error!("The template {}, doesn't appear to exist locally", template);
                    exit(1);
                }
            } else {
                error!("The template {}, doesn't appear to exist locally", template);
                exit(1);
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
    let mut angreal_path = String::new();
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

    let file_contents = match fs::read_to_string(toml) {
        Ok(c) => c,
        Err(e) => {
            error!("{:?}", e);
            exit(1);
        }
    };

    // build our tera context from toml file.
    let extract = file_contents.parse::<Table>().unwrap();
    let mut context = Context::new();
    let mut toml_values = Map::new();
    for (k, v) in extract.iter() {
        let value = if v.is_str()
            && v.as_str().unwrap().starts_with("{{")
            && v.as_str().unwrap().contains("}}")
        {
            let temp_value = v.clone();
            let rendered_value =
                Tera::one_off(temp_value.as_str().unwrap(), &context, false).unwrap();
            Value::from(rendered_value)
        } else {
            v.clone()
        };

        let input = if take_input {
            print!("{}? [{}]: ", k, value);
            read!("{}\n")
        } else {
            String::new()
        };

        if input.trim().is_empty() | take_input.not() {
            if value.is_str() {
                context.insert(k, &value.as_str().unwrap());
                toml_values.insert(k.into(), value.clone());
            }
            if value.is_integer() {
                context.insert(k, &value.as_integer().unwrap());
                toml_values.insert(k.into(), value.clone());
            }
            if value.is_bool() {
                context.insert(k, &value.as_bool().unwrap());
                toml_values.insert(k.into(), value.clone());
            }
            if value.is_float() {
                context.insert(k, &value.as_float().unwrap());
                toml_values.insert(k.into(), value.clone());
            }
        } else {
            if value.is_str() {
                context.insert(k, &input.trim());
                toml_values.insert(k.into(), Value::String(input.trim().to_string()));
            }
            if value.is_integer() {
                context.insert(k, &input.trim().parse::<i32>().unwrap());
                toml_values.insert(
                    k.into(),
                    Value::Integer(input.trim().parse::<i64>().unwrap()),
                );
            }
            if value.is_bool() {
                context.insert(k, &input.trim());
                toml_values.insert(
                    k.into(),
                    Value::Boolean(input.trim().parse::<bool>().unwrap()),
                );
            }
            if value.is_float() {
                context.insert(k, &input.trim().parse::<f64>().unwrap());
                toml_values.insert(k.into(), Value::Float(input.trim().parse::<f64>().unwrap()));
            }
        }
    }

    // first we create a Tera instance from an empty directory so we can extend it
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push(Path::new("angreal_tmp"));

    if tmp_dir.is_dir().not() {
        debug!("Creating tmpdir at {:?}", tmp_dir);
        fs::create_dir(&tmp_dir).unwrap();
    }

    tmp_dir.push(Path::new("*"));
    let mut tera = Tera::new(tmp_dir.to_str().unwrap()).unwrap();

    tmp_dir.pop();
    if tmp_dir.is_dir() {
        debug!("Destroying tmpdir at {:?}", tmp_dir);
        fs::remove_dir_all(&tmp_dir).unwrap();
    }

    // We get our templates glob
    let mut template = <&std::path::Path>::clone(&path).to_path_buf();
    template.push(Path::new("**/*"));

    // And build our full prefix
    let _template_name = <&std::path::Path>::clone(&path).file_name().unwrap();

    for file in glob(template.to_str().unwrap()).expect("Failed to read glob pattern") {
        let file_path = file.as_ref().unwrap();
        let rel_path = file_path.strip_prefix(path).unwrap().to_str().unwrap();

        if file.as_ref().unwrap().is_file() && rel_path.starts_with("{{") && rel_path.contains("}}")
        {
            debug!(
                "Adding template with relative path {:?} to tera instance.",
                rel_path
            );
            tera.add_template_file(file.as_ref().unwrap().to_str().unwrap(), Some(rel_path))
                .unwrap();
        }
    }

    // build our directory structure first
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| e.file_type().is_dir()) {
        let path_template = entry.unwrap().clone();
        let path_postfix = path_template.path();
        let path_template = path_postfix.strip_prefix(path).unwrap().to_str().unwrap();

        if path_template.starts_with("{{") && path_template.contains("}}") {
            let real_path = Tera::one_off(path_template, &context, false).unwrap();

            if Path::new(real_path.as_str()).is_dir() & force.not() {
                error!(
                    "{} already exists. Will not proceed unless `--force`/force=True is used.",
                    real_path.as_str()
                );
                exit(1)
            }
            if real_path.starts_with('.') {
                //skip any sort of top level dot files - extend with an exclusion glob in the future
                // todo: exclusion glob
                continue;
            }

            if real_path.ends_with(".angreal") {
                angreal_path = real_path.clone();
            }

            debug!("Creating directory {:?}", real_path);
            fs::create_dir(real_path.as_str()).unwrap();
        }
    }

    // render templates
    for template in tera.get_template_names() {
        if template == "angreal.toml" {
            // never render the angreal.toml
            // todo: exclusion glob
            continue;
        }

        if template.starts_with('.') {
            // we don't render dot files either
            // todo: exclusion glob
            continue;
        }

        let rendered = tera.render(template, &context).unwrap();
        let path = Tera::one_off(template, &context, false).unwrap();
        debug!("Rendering file at {:?}", path);
        let mut output = File::create(path).unwrap();
        write!(output, "{}", rendered.as_str()).unwrap();
    }

    let toml_string = toml::to_string(&Value::Table(toml_values.clone())).unwrap();
    let mut value_path = PathBuf::new();
    value_path.push(angreal_path.as_str());
    value_path.push("angreal.toml");

    let mut output = File::create(&value_path).unwrap();
    write!(output, "{}", toml_string.as_str()).unwrap();
    debug!("Storing initialization values to {}", &value_path.display());
    angreal_path

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
