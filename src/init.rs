use git2::Repository;
use git_url_parse::{GitUrl, Scheme};
use home::home_dir;


use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::process::exit;
use std::{fs};
use std::{
    path::{Path, PathBuf},
};
use text_io::read;
use toml::Value;
use walkdir::{WalkDir};

use tera::{Context, Tera};

pub fn init(template: &str, force: bool, use_defaults: bool){

    let angreal_home = create_home_dot_angreal();
    let template_type = get_scheme(template).unwrap();

    
    let template = match template_type.as_str() {
        "https"|"gitssh"|"ssh"|"git" => {
            let remote = GitUrl::parse(template).unwrap();
            let mut dst = angreal_home.clone();
            dst.push(remote.name.as_str());
            Repository::clone(template, &dst).unwrap().path().to_path_buf()
        },
        "file" => {
            let mut try_template = angreal_home;
            try_template.push(Path::new(template));

            if try_template.is_dir(){
                Repository::open(try_template).unwrap().path().to_path_buf()
            } else {
                exit(1);
            }
        },
        &_ =>{
            exit(1);
        }
    };

    render_template(Path::new(&template), use_defaults, force);

}

fn get_scheme(u: &str) -> Result<String, ()> {
    let s = GitUrl::parse(u.clone()).unwrap();

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
    home_dir.push(".angreal");

    if home_dir.exists().not() {
        fs::create_dir(&home_dir).unwrap();
    }

    home_dir
}



fn render_template(path: &Path, take_input: bool, force: bool) {
    
    // Build our context from the toml/CLI
    let mut toml = path.clone().to_path_buf();
    toml.push(Path::new("angreal.toml"));
    let file_contents = match fs::read_to_string(toml) {
        Ok(c) => c,
        Err(_) => {
            //LOG ERROR
            exit(1);
        }
    };
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

    let mut template = path.clone().to_path_buf();
    template.push(Path::new("**/*"));
    let tera = Tera::new(template.to_str().unwrap()).unwrap();

    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| e.file_type().is_dir()) {
        let path_template = entry.unwrap().clone();
        let path_postfix = path_template.path();
        let path_template = path_postfix.strip_prefix(path).unwrap().to_str().unwrap();
        let real_path = Tera::one_off(path_template, &context, false).unwrap();
        
        if Path::new(real_path.as_str()).is_dir() & force.not() {
            exit(1)
        }
        if real_path.starts_with("."){
            continue;
        }
        fs::create_dir(real_path.as_str());
    }

    for template in tera.get_template_names() {
        if template == "angreal.toml"{
            continue;
        }

        if template.starts_with("."){
            continue;
        }

        println!("{:?}",template);
        let rendered = tera.render(template, &context).unwrap();
        let path = Tera::one_off(template, &context, false).unwrap();

        let mut output = File::create(path).unwrap();
        write!(output, "{}", rendered.as_str());
        
    }
}

#[cfg(test)]
#[path = "../tests"]
mod tests {
    use std::path::{Path, PathBuf};
    use git2::Repository;
    use std::env;
    

    mod common;

    #[test]
    fn test_render_template() {
        let mut template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        template_root.push(Path::new("tests/common/test_assets/test_template"));
        crate::init::render_template(&template_root, false, true);
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
