use crate::git::{git_clone, git_pull_ff};

use git_url_parse::{GitUrl, Scheme};
use home::home_dir;

use glob::glob;
use log::error;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::process::exit;
use tera::{Context, Tera};
use text_io::read;
use toml::Value;
use walkdir::WalkDir;

/// Initialize a new project by rendering a template.
/// If we wish a full over write use force == True
/// If we wish to use the angreal.toml defaults take_inputs == False
pub fn init(template: &str, force: bool, take_inputs: bool) {
    let angreal_home = create_home_dot_angreal();
    let template_type = get_scheme(template).unwrap();

    // todo - implement a local file system template branch
    // "file" should cover the following scenarios
    // - a template that already exists at ~/.angreal, "ff_pull" and go
    // - a filesystem git repo, clone and go
    // - a file not in ~/.angreal, "copy?" and go
    let template = match template_type.as_str() {
        "https" | "gitssh" | "ssh" | "git" => {
            // If we get a git url , go get it either by a clone if it doesn't
            // already exist, or as a ff pull if it does
            let remote = GitUrl::parse(template).unwrap();
            let mut dst = angreal_home;
            dst.push(remote.name.as_str());

            if dst.is_dir() {
                git_pull_ff(dst.to_str().unwrap())
            } else {
                git_clone(template, dst.to_str().unwrap())
            }
        }
        "file" => {
            // if someone runs `angreal init template`, we check ~/.angreal/template
            // for the template to use and attempt a ff-pull on that repo
            let mut try_template = angreal_home;
            try_template.push(Path::new(template));

            //  First we try ~/.angreal for a teamplate with that name
            if try_template.is_dir() {
                git_pull_ff(try_template.to_str().unwrap())
            } else if Path::new(template).is_dir() {
                // then we see if it's just a local angreal template
                let mut angreal_toml = Path::new(template).to_path_buf();
                angreal_toml.push("angreal.toml");

                if angreal_toml.is_file() {
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
    render_template(Path::new(&template), take_inputs, force);
}

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

fn create_home_dot_angreal() -> PathBuf {
    let mut home_dir = home_dir().unwrap();
    home_dir.push(".angrealrc");

    if home_dir.exists().not() {
        fs::create_dir(&home_dir).unwrap();
    }

    home_dir
}

fn render_template(path: &Path, take_input: bool, force: bool) {
    // Verify the provided template path is minimially compliant.
    let mut toml = path.to_path_buf();
    toml.push(Path::new("angreal.toml"));

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
    let value = file_contents.parse::<Value>().unwrap();
    let extract = value.as_table().unwrap();
    let mut context = Context::new();
    for (k, v) in extract.iter() {
        let input = if take_input {
            println!("{}? [{}]", k, v);
            read!("{}\n")
        } else {
            String::new()
        };

        if input.trim().is_empty() | take_input.not() {
            if v.is_str() {
                context.insert(k, &v.as_str().unwrap());
            }
            if v.is_integer() {
                context.insert(k, &v.as_integer().unwrap());
            }
            if v.is_bool() {
                context.insert(k, &v.as_bool().unwrap());
            }
            if v.is_float() {
                context.insert(k, &v.as_float().unwrap());
            }
        } else {
            if v.is_str() {
                context.insert(k, &input.as_str());
            }
            if v.is_integer() {
                context.insert(k, &input.parse::<i32>().unwrap());
            }
            if v.is_bool() {
                context.insert(k, &input.as_str());
            }
            if v.is_float() {
                context.insert(k, &input.parse::<f64>().unwrap());
            }
        }
    }

    // first we create a Tera instance from an empty directory so we can extend it
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push(Path::new("angreal_tmp"));
    fs::create_dir(&tmp_dir); // we don't unwrap/check because we know and expect this might exist
    tmp_dir.push(Path::new("*"));
    let mut tera = Tera::new(tmp_dir.to_str().unwrap()).unwrap();
    // fs::remove_dir_all(&tmp_dir).unwrap();

    // We get our templates glob
    let mut template = path.clone().to_path_buf();
    template.push(Path::new("**/*"));

    // And build our full prefix
    let _template_name = path.clone().file_name().unwrap();

    for file in glob(template.to_str().unwrap()).expect("Failed to read glob pattern") {
        let file_path = file.as_ref().unwrap();
        let rel_path = file_path.strip_prefix(path).unwrap().to_str().unwrap();

        if file.as_ref().unwrap().is_file() {
            if rel_path.starts_with("{{") && rel_path.contains("}}") {
                tera.add_template_file(file.as_ref().unwrap().to_str().unwrap(), Some(rel_path)).unwrap();
            }
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
            // we don't render dot files eiterh
            // todo: exclusion glob
            continue;
        }

        let rendered = tera.render(template, &context).unwrap();
        let path = Tera::one_off(template, &context, false).unwrap();

        let mut output = File::create(path).unwrap();
        write!(output, "{}", rendered.as_str()).unwrap();
    }
}

#[cfg(test)]
#[path = "../tests"]
mod tests {

    use std::ops::Not;
    use std::path::{Path, PathBuf};
    use std::{env, fs};

    mod common;

    #[test]
    fn test_init_from_git() {
        crate::init::init(
            "https://gitlab.com/angreal/angreal2_test_template.git",
            true,
            false,
        );
        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("angreal_test_project"));
        fs::remove_dir_all(&rendered_root);

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
        let assets_no_exists = assets.is_dir().not();

        let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rendered_root.push(Path::new("folder_name"));
        let rendered_root_exists = rendered_root.is_dir();

        let mut dot_angreal = rendered_root.clone();
        dot_angreal.push(".angreal");
        println!("{:?}", dot_angreal);
        let dot_angreal_exists = dot_angreal.is_dir();

        let mut readme_rst = rendered_root.clone();
        readme_rst.push("README.rst");
        let readme_rst_exists = readme_rst.is_file();

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
        let url_https = "https://gitlab.com/angreal/angreal.git";
        let url_ssh = "git@gitlab.com:angreal/angreal.git";
        let url_git = "git:gitlab.com/angreal/angreal.git";
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
