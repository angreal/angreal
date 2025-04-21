//! Git operations for Angreal
use git2::{FetchOptions, RemoteCallbacks, Repository};
use git2_credentials::CredentialHandler;
use log::info;
use std::path::PathBuf;

/// Check if a remote repository exists
pub fn remote_exists(remote: &str) -> bool {
    let mut callbacks = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut handler = CredentialHandler::new(git_config);
    callbacks.credentials(move |url, username, allowed| {
        handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let repo = Repository::init(".").unwrap();
    let result = repo.find_remote(remote).is_ok();
    drop(repo);
    result
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
    let mut callbacks = RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut handler = CredentialHandler::new(git_config);
    callbacks.credentials(move |url, username, allowed| {
        handler.try_next_credential(url, username, allowed)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let _repo = match Repository::clone(remote, ".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    PathBuf::from(".")
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

    match remote.fetch(&["master"], Some(&mut fetch_options), None) {
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
        let refname = format!("refs/heads/{}", "master");
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
