//! Git operations for Angreal
use git2::{FetchOptions, RemoteCallbacks, Repository};
use git2_credentials::CredentialHandler;
use log::info;
use std::path::PathBuf;
use tempfile::tempdir;

/// Check if a remote repository exists
pub fn remote_exists(remote: &str) -> bool {
    // Create a temporary directory for the test
    let temp_dir = match tempdir() {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Failed to create temporary directory: {}", e);
            return false;
        }
    };

    // Initialize a new repository in the temp directory
    let repo = match Repository::init(temp_dir.path()) {
        Ok(repo) => repo,
        Err(e) => {
            log::error!("Failed to initialize repository: {}", e);
            return false;
        }
    };

    // Configure fetch options without authentication for public repositories
    let mut fetch_options = FetchOptions::new();

    // Only set up authentication if it's an SSH URL
    if remote.starts_with("git@") {
        let mut callbacks = RemoteCallbacks::new();
        let git_config = match git2::Config::open_default() {
            Ok(config) => config,
            Err(e) => {
                log::error!("Failed to open git config: {}", e);
                return false;
            }
        };
        let mut handler = CredentialHandler::new(git_config);
        callbacks.credentials(move |url, username, allowed| {
            handler.try_next_credential(url, username, allowed)
        });
        fetch_options.remote_callbacks(callbacks);
    }

    // Try to fetch from the remote
    let mut remote = match repo.remote_anonymous(remote) {
        Ok(remote) => remote,
        Err(e) => {
            log::debug!("Failed to create remote: {}", e);
            return false;
        }
    };

    // Attempt to fetch from the remote
    match remote.fetch(
        &["refs/heads/*:refs/heads/*"],
        Some(&mut fetch_options),
        None,
    ) {
        Ok(_) => true, // Fetch succeeded, repository exists
        Err(e) => {
            // Check if the error is due to repository not existing
            if e.code() == git2::ErrorCode::NotFound {
                false
            } else {
                log::debug!("Unexpected error while checking remote: {}", e);
                false
            }
        }
    }
}

/// Clone a git repository to a specific location
pub fn git_clone(remote: &str, path: &str) -> PathBuf {
    let mut callbacks = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut handler = CredentialHandler::new(git_config);
    callbacks.credentials(move |url, username, allowed| {
        handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let _repo = match Repository::clone(remote, path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    PathBuf::from(path)
}

/// Clone a git repository to the current directory
pub fn git_clone_here(remote: &str) -> PathBuf {
    // Extract repository name from remote URL
    let repo_name = if remote.starts_with("git@") {
        // SSH format: git@github.com:user/repo.git
        remote
            .split(':')
            .next_back()
            .and_then(|s| s.strip_suffix(".git"))
            .unwrap_or_else(|| {
                panic!("Invalid SSH remote URL format: {}", remote);
            })
    } else {
        // HTTPS format: https://github.com/user/repo.git
        remote
            .split('/')
            .next_back()
            .and_then(|s| s.strip_suffix(".git"))
            .unwrap_or_else(|| {
                panic!("Invalid HTTPS remote URL format: {}", remote);
            })
    };

    // Create the subdirectory
    std::fs::create_dir(repo_name).unwrap_or_else(|e| {
        panic!("Failed to create directory {}: {}", repo_name, e);
    });

    let mut callbacks = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut handler = CredentialHandler::new(git_config);
    callbacks.credentials(move |url, username, allowed| {
        handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let _repo = match Repository::clone(remote, repo_name) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    PathBuf::from(repo_name)
}

/// Pull changes from a remote repository
pub fn git_pull_ff(path: &str) -> PathBuf {
    let mut callbacks = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut handler = CredentialHandler::new(git_config);
    callbacks.credentials(move |url, username, allowed| {
        handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(e) => panic!("Failed to find remote: {}", e),
    };

    match remote.fetch(&["main"], Some(&mut fetch_options), None) {
        Ok(_) => (),
        Err(e) => panic!("Failed to fetch: {}", e),
    }

    let fetch_head = match repo.find_reference("FETCH_HEAD") {
        Ok(head) => head,
        Err(e) => panic!("Failed to find FETCH_HEAD: {}", e),
    };

    let fetch_commit = match repo.reference_to_annotated_commit(&fetch_head) {
        Ok(commit) => commit,
        Err(e) => panic!("Failed to find commit: {}", e),
    };

    let analysis = match repo.merge_analysis(&[&fetch_commit]) {
        Ok(analysis) => analysis,
        Err(e) => panic!("Failed to analyze merge: {}", e),
    };

    if analysis.0.is_up_to_date() {
        info!("Already up-to-date");
    } else if analysis.0.is_fast_forward() {
        info!("Performing fast-forward");
        let refname = format!("refs/heads/{}", "main");
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                r.set_target(fetch_commit.id(), "Fast-Forward").unwrap();
            }
            Err(_) => {
                repo.reference(&refname, fetch_commit.id(), true, "Fast-Forward")
                    .unwrap();
            }
        }
        repo.set_head(&refname).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .unwrap();
    } else {
        panic!("Can't fast-forward");
    }

    PathBuf::from(path)
}
