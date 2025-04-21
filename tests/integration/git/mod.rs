use angreal::git::{git_clone, git_clone_here, git_pull_ff, remote_exists};
use same_file::is_same_file;
use std::fs;

#[test]
fn test_repo_exists() {
    let remote = "https://github.com/angreal/angreal_test_template.git";
    let response = remote_exists(remote);
    assert_eq!(response, true);
}

#[test]
fn test_repo_no_exists() {
    let remote = "https://github.com/angreal/no_angreal_test_template.git";
    let response = remote_exists(remote);
    assert_eq!(response, false);
}

#[test]
fn test_clone_public() {
    let mut tmp_dir = tempfile::tempdir().unwrap().into_path();
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
    let mut tmp_dir = tempfile::tempdir().unwrap().into_path();
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
    let mut tmp_dir = tempfile::tempdir().unwrap().into_path();
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
    let mut tmp_dir = tempfile::tempdir().unwrap().into_path();
    tmp_dir.push("angreal_test_template");
    let remote = "https://github.com/angreal/angreal_test_template.git";

    let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

    let local = git_pull_ff(local_repo.to_str().unwrap());

    let equality_test = is_same_file(&tmp_dir, local).unwrap();
    fs::remove_dir_all(&tmp_dir).unwrap_or(());
    assert!(equality_test);
}

/// We skip this test on windows because github action is broken
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_git_pull_ff_private() {
    let mut tmp_dir = tempfile::tempdir().unwrap().into_path();
    tmp_dir.push("angreal_test_template");
    let remote = "git@github.com:angreal/private_test_template.git";

    let local_repo = git_clone(remote, tmp_dir.to_str().unwrap());

    let local = git_pull_ff(local_repo.to_str().unwrap());

    let equality_test = is_same_file(&tmp_dir, local).unwrap();
    fs::remove_dir_all(&tmp_dir).unwrap_or(());
    assert!(equality_test);
}
