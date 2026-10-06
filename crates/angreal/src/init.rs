//! The angreal `init` command.
//!
use crate::{
    git::{git_clone, remote_exists, try_git_pull_ff},
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

use log::{debug, error, warn};

/// Initialize a new project by rendering a template.
pub fn init(
    template: &str,
    force: bool,
    take_inputs: bool,
    values_file: Option<&str>,
    in_place: bool,
    insecure: bool,
) {
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
        "oci" => handle_oci_template(template, &angreal_home, insecure),
        &_ => {
            error!(
                "Unhandled template type {} from {}, exiting.",
                template_type.as_str(),
                template
            );
            exit(1);
        }
    };

    let rendered_dot_angreal_path = render_template(
        Path::new(&template),
        take_inputs,
        force,
        values_file,
        in_place,
    );

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
    // Short-circuit local filesystem paths before URL parsing.
    // git_url_parse rejects Windows-style absolute paths (drive letter +
    // backslashes, e.g. C:\Users\...\template) and would panic the caller.
    // If the input names an extant directory on disk, classify it as "file"
    // without going through URL parsing at all.
    if Path::new(u).is_dir() {
        return Ok("file".to_string());
    }

    // OCI references carry an explicit `oci://` scheme. Short-circuit before git
    // URL parsing, which does not understand it.
    if u.starts_with("oci://") {
        return Ok("oci".to_string());
    }

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

/// Bring a template cached under `~/.angrealrc` up to date with a fast-forward
/// pull. If the pull fails (no network, a refused key, a diverged branch), warn
/// and use the cached copy as it is.
fn refresh_cached_template(path: &Path) -> PathBuf {
    match try_git_pull_ff(&path.to_string_lossy()) {
        Ok(p) => p,
        Err(e) => {
            warn!(
                "Could not update the cached template at {}: {}. Using the cached copy.",
                path.display(),
                e
            );
            path.to_path_buf()
        }
    }
}

fn handle_file_template(template: &str, angreal_home: &Path) -> String {
    let mut try_template = angreal_home.to_path_buf();
    try_template.push(Path::new(template));

    // Only a relative name can refer to a cached template. An absolute path
    // replaces the base in `push`, so without this check a local git checkout
    // given by its absolute path would be treated as a cache entry and pulled.
    if Path::new(template).is_relative() && try_template.is_dir() {
        let mut git_location = try_template.clone();
        git_location.push(Path::new(".git"));

        if git_location.exists() {
            debug!("Template exists at {:?}, attempting ff-pull.", try_template);
            refresh_cached_template(&try_template)
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
                refresh_cached_template(&try_supported)
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
        refresh_cached_template(&dst);
    } else {
        debug!("Template does not exist, attempting clone to {:?}", dst);
        git_clone(template, dst.to_str().unwrap());
    }

    dst
}

/// Pull an OCI template artifact into the local cache and return the unpacked
/// template directory, ready for rendering.
///
/// The cache lives at `~/.angrealrc/oci/<registry>/<repository>/<tag>/`. As with
/// git templates (which ff-pull), the artifact is re-fetched each run so a moved
/// tag is honored; the cache dir is cleared first to avoid mixing in stale files
/// from a previous pull of the same tag.
fn handle_oci_template(template: &str, angreal_home: &Path, insecure: bool) -> PathBuf {
    use crate::integrations::oci;

    let subpath = match oci::cache_subpath(template) {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };
    let dst = angreal_home.join("oci").join(subpath);

    if dst.exists() {
        debug!("OCI template cache exists at {:?}, refreshing.", dst);
        if let Err(e) = fs::remove_dir_all(&dst) {
            error!("Failed to clear OCI template cache at {:?}: {}", dst, e);
            exit(1);
        }
    }

    debug!("Pulling OCI template {} into {:?}", template, dst);
    match oci::Oci::pull_artifact(template, &dst, &oci::resolve_auth(template), insecure) {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to pull OCI template {}: {}", template, e);
            exit(1);
        }
    }
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
    in_place: bool,
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
    let rendered_files = render_dir(path, context, &env::current_dir().unwrap(), force, in_place);

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
mod tests {
    use super::get_scheme;

    #[test]
    fn get_scheme_detects_oci() {
        assert_eq!(
            get_scheme("oci://ghcr.io/angreal/python:latest").unwrap(),
            "oci"
        );
        assert_eq!(get_scheme("oci://localhost:5000/demo:v1").unwrap(), "oci");
    }

    #[test]
    fn get_scheme_still_detects_git_https() {
        assert_eq!(
            get_scheme("https://github.com/angreal/angreal.git").unwrap(),
            "https"
        );
    }

    mod file_templates {
        use super::super::handle_file_template;
        use git2::{Repository, RepositoryInitOptions, Signature};
        use std::fs;
        use std::path::Path;
        use tempfile::TempDir;

        /// A repository on `main` with one commit that adds `angreal.toml`.
        fn template_repo(path: &Path) -> Repository {
            let mut opts = RepositoryInitOptions::new();
            opts.initial_head("main");
            let repo = Repository::init_opts(path, &opts).unwrap();
            commit_file(&repo, "angreal.toml", "key = \"v1\"\n", "first");
            repo
        }

        fn commit_file(repo: &Repository, name: &str, contents: &str, msg: &str) {
            let root = repo.workdir().unwrap().to_path_buf();
            fs::write(root.join(name), contents).unwrap();
            let mut index = repo.index().unwrap();
            index.add_path(Path::new(name)).unwrap();
            index.write().unwrap();
            let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
            let sig = Signature::now("test", "test@example.com").unwrap();
            let parents: Vec<git2::Commit> = match repo.head() {
                Ok(h) => vec![h.peel_to_commit().unwrap()],
                Err(_) => vec![],
            };
            let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
            repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parent_refs)
                .unwrap();
        }

        #[test]
        fn absolute_local_checkout_is_rendered_as_is_without_a_pull() {
            let home = TempDir::new().unwrap();
            let work = TempDir::new().unwrap();
            let checkout = work.path().join("my-template");
            let repo = template_repo(&checkout);
            // An origin that cannot be reached: any fetch would fail.
            repo.remote("origin", "/nonexistent/angreal/remote.git")
                .unwrap();
            fs::write(checkout.join("angreal.toml"), "key = \"local edit\"\n").unwrap();

            let abs = checkout.to_str().unwrap();
            let resolved = handle_file_template(abs, home.path());

            assert_eq!(resolved, abs);
            // The uncommitted local edit is untouched.
            assert_eq!(
                fs::read_to_string(checkout.join("angreal.toml")).unwrap(),
                "key = \"local edit\"\n"
            );
        }

        #[test]
        fn absolute_local_checkout_is_not_pulled_even_when_origin_is_ahead() {
            let home = TempDir::new().unwrap();
            let origin_dir = TempDir::new().unwrap();
            let origin = template_repo(origin_dir.path());
            let work = TempDir::new().unwrap();
            let checkout = work.path().join("my-template");
            Repository::clone(origin_dir.path().to_str().unwrap(), &checkout).unwrap();
            // Origin moves ahead, and the author has an uncommitted edit. A
            // fast-forward pull with a forced checkout would replace the edit.
            commit_file(&origin, "angreal.toml", "key = \"v2\"\n", "second");
            fs::write(checkout.join("angreal.toml"), "key = \"local edit\"\n").unwrap();

            let abs = checkout.to_str().unwrap();
            let resolved = handle_file_template(abs, home.path());

            assert_eq!(resolved, abs);
            assert_eq!(
                fs::read_to_string(checkout.join("angreal.toml")).unwrap(),
                "key = \"local edit\"\n"
            );
        }

        #[test]
        fn cached_template_is_still_fast_forwarded() {
            let home = TempDir::new().unwrap();
            let origin_dir = TempDir::new().unwrap();
            let origin = template_repo(origin_dir.path());
            let cached = home.path().join("cached-template");
            Repository::clone(origin_dir.path().to_str().unwrap(), &cached).unwrap();
            commit_file(&origin, "angreal.toml", "key = \"v2\"\n", "second");

            let resolved = handle_file_template("cached-template", home.path());

            assert_eq!(Path::new(&resolved), cached.as_path());
            assert_eq!(
                fs::read_to_string(cached.join("angreal.toml")).unwrap(),
                "key = \"v2\"\n"
            );
        }

        #[test]
        fn failed_pull_of_a_cached_template_uses_the_cached_copy() {
            let home = TempDir::new().unwrap();
            let cached = home.path().join("cached-template");
            let repo = template_repo(&cached);
            repo.remote("origin", "/nonexistent/angreal/remote.git")
                .unwrap();

            let resolved = handle_file_template("cached-template", home.path());

            assert_eq!(Path::new(&resolved), cached.as_path());
            assert_eq!(
                fs::read_to_string(cached.join("angreal.toml")).unwrap(),
                "key = \"v1\"\n"
            );
        }
    }
}
