//! Basic functions to git repos
use git2::Repository;
use git2_credentials::CredentialHandler;
use git_url_parse::GitUrl;
use log::error;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Attempts to clone a repo within the current working directory.
/// Mimics the command `git clone repo`
/// ```ignore
/// let clone_path = git_clone_here("http://github.com/remote/test.git")
/// ```
pub fn git_clone_here(remote: &str) -> PathBuf {
    let remote_url = GitUrl::parse(remote).unwrap();
    git_clone(remote, remote_url.name.as_str())
}

/// Attempts to clone a repo to a source directory
/// The full path to the target repo needs to be provided, i.e.
/// if the repo is https://github.com/test.git you would probably want to
/// provide a target as /path/to/clone/test
/// ```ignore
/// let clone_path = git_clone("http://github.com/remote/test.git","path/to/target/test")
/// ```
pub fn git_clone(remote: &str, local: &str) -> PathBuf {
    let into = Path::new(local);

    let mut cb = git2::RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut ch = CredentialHandler::new(git_config);
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options
        .remote_callbacks(cb)
        .download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);

    git2::build::RepoBuilder::new()
        .branch("main")
        .fetch_options(fetch_options)
        .clone(remote, into)
        .unwrap()
        .workdir()
        .unwrap()
        .to_path_buf()
}

/// Attempts a fast forward pull on a pre existing repo
/// ```ignore
/// let pull_path = git_pull_ff("path/to/target/test")
/// ```
pub fn git_pull_ff(repo: &str) -> PathBuf {
    let mut cb = git2::RemoteCallbacks::new();
    let git_config = git2::Config::open_default().unwrap();
    let mut ch = CredentialHandler::new(git_config);
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options
        .remote_callbacks(cb)
        .download_tags(git2::AutotagOption::All)
        .update_fetchhead(true);

    let repo = Repository::open(repo).unwrap();
    repo.find_remote("origin")
        .unwrap()
        .fetch(&["main"], Some(&mut fetch_options), None)
        .unwrap();
    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();
    if analysis.0.is_up_to_date() {
        repo.workdir().unwrap().to_path_buf()
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "main");
        let mut reference = repo.find_reference(&refname).unwrap();
        reference
            .set_target(fetch_commit.id(), "Fast-Forward")
            .unwrap();
        repo.set_head(&refname).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .unwrap();
        repo.workdir().unwrap().to_path_buf()
    } else {
        error!("Fast forward pull not available on {}, suggest manually deleting and re cloning the repo.", repo.workdir().unwrap().to_str().unwrap() );
        exit(127);
    }
}

// TODO: something caused these tests to start failing on the Mac runners (and locally) with the last mac update. AFAICT - this all still works correctly but the
// local repo is returning a full path (/private/var) while tmp_dir is returning the symlink (/var/).
#[cfg(test)]
#[path = "../../tests"]
mod tests {
    use super::*;
    use std::fs;
    mod common;
    use same_file::is_same_file;
    #[test]
    fn test_clone_public() {
        let mut tmp_dir = common::make_tmp_dir();
        let remote = "https://github.com/angreal/angreal_test_template.git";
        tmp_dir.push("angreal_test_template");
        let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

        let equality_test = is_same_file(local_repo, &tmp_dir).unwrap();
        fs::remove_dir_all(&tmp_dir).unwrap_or(());

        assert!(equality_test);
    }

    /// we skip this test on windows because the gitlab runner is broken
    #[test]
    #[cfg_attr(target_os = "windows", ignore)]
    fn test_clone_private() {
        let mut tmp_dir = common::make_tmp_dir();
        let remote = "git@github.com:angreal/private_test_template.git";
        tmp_dir.push("angreal_test_template");
        let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

        let equality_test = is_same_file(local_repo, &tmp_dir).unwrap();
        fs::remove_dir_all(&tmp_dir).unwrap_or(());

        assert!(equality_test);
    }

    #[test]
    fn test_clone_here() {
        let starting_dir = std::env::current_dir().unwrap();
        let mut tmp_dir = common::make_tmp_dir();
        std::env::set_current_dir(&tmp_dir).unwrap_or(());

        let remote = "https://github.com/angreal/angreal_test_template.git";
        let path = git_clone_here(remote);
        tmp_dir.push("angreal_test_template");

        let equality_test = is_same_file(path, &tmp_dir).unwrap();
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
        std::env::set_current_dir(starting_dir).unwrap_or(());
        assert!(equality_test);
    }

    #[test]
    fn test_git_pull_ff() {
        let mut tmp_dir = common::make_tmp_dir();
        tmp_dir.push("angreal_test_template");
        let remote = "https://github.com/angreal/angreal_test_template.git";

        let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

        let local = git_pull_ff(local_repo.to_str().unwrap());

        let equality_test = is_same_file(&tmp_dir, &local).unwrap();
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
        assert!(equality_test);
    }

    /// We skip this test on windows because github action is broken
    #[test]
    #[cfg_attr(target_os = "windows", ignore)]
    fn test_git_pull_ff_private() {
        let mut tmp_dir = common::make_tmp_dir();
        tmp_dir.push("angreal_test_template");
        let remote = "git@github.com:angreal/private_test_template.git";

        let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

        let local = git_pull_ff(local_repo.to_str().unwrap());

        let equality_test = is_same_file(&tmp_dir, &local).unwrap();
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
        assert!(equality_test);
    }
}
