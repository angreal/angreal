use self::utils::GitTestFixture;
use angreal::git::{git_clone, git_clone_here, git_pull_ff, remote_exists};
use same_file::is_same_file;

mod utils;

/// Test suite for remote repository existence checks
#[cfg(test)]
mod remote_tests {
    use super::*;

    #[test]
    fn test_repo_exists() {
        let remote = "https://github.com/angreal/angreal_test_template.git";
        assert!(remote_exists(remote), "Public repository should exist");
    }

    #[test]
    fn test_repo_no_exists() {
        let remote = "https://github.com/angreal/no_angreal_test_template.git";
        assert!(
            !remote_exists(remote),
            "Non-existent repository should not be found"
        );
    }
}

/// Test suite for repository cloning operations
#[cfg(test)]
mod clone_tests {
    use super::*;

    #[test]
    fn test_clone_public() {
        let fixture = GitTestFixture::new();
        let remote = "https://github.com/angreal/angreal_test_template.git";

        let clone_dir = fixture.create_subdir("angreal_test_template");
        let local_repo = git_clone(remote, clone_dir.to_str().unwrap());

        assert!(
            is_same_file(local_repo, &clone_dir).unwrap(),
            "Cloned repository should match target directory"
        );
    }

    /// Test cloning a private repository (requires authentication)
    #[test]
    #[cfg_attr(target_os = "windows", ignore)]
    #[should_panic(expected = "authentication required")]
    fn test_clone_private() {
        let fixture = GitTestFixture::new();
        let remote = "git@github.com:angreal/private_test_template.git";

        let clone_dir = fixture.create_subdir("angreal_test_template");
        git_clone(remote, clone_dir.to_str().unwrap());
    }

    #[test]
    fn test_clone_here() {
        let fixture = GitTestFixture::new();
        fixture.change_to_temp_dir();

        let remote = "https://github.com/angreal/angreal_test_template.git";
        let path = git_clone_here(remote);

        // Debug: Print the paths and check if they exist
        println!("Cloned path: {:?}", path);
        println!("Path exists: {}", path.exists());
        println!("Path is dir: {}", path.is_dir());

        let expected_dir = fixture.temp_path().join("angreal_test_template");
        println!("Expected dir: {:?}", expected_dir);
        println!("Expected dir exists: {}", expected_dir.exists());
        println!("Expected dir is dir: {}", expected_dir.is_dir());

        // List contents of current directory
        println!("Current directory contents:");
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {:?}", entry.path());
                }
            }
        }

        assert!(
            is_same_file(path, &expected_dir).unwrap(),
            "Cloned repository should match expected directory"
        );
    }
}

/// Test suite for pull operations
#[cfg(test)]
mod pull_tests {
    use super::*;

    #[test]
    fn test_git_pull_ff() {
        let fixture = GitTestFixture::new();
        let remote = "https://github.com/angreal/angreal_test_template.git";

        let clone_dir = fixture.create_subdir("angreal_test_template");
        let local_repo = git_clone(remote, clone_dir.to_str().unwrap());
        let pulled_repo = git_pull_ff(local_repo.to_str().unwrap());

        assert!(
            is_same_file(&clone_dir, pulled_repo).unwrap(),
            "Pulled repository should match original directory"
        );
    }

    /// Test pulling from a private repository (requires authentication)
    #[test]
    #[cfg_attr(target_os = "windows", ignore)]
    #[should_panic(expected = "authentication required")]
    fn test_git_pull_ff_private() {
        let fixture = GitTestFixture::new();
        let remote = "git@github.com:angreal/private_test_template.git";

        let clone_dir = fixture.create_subdir("angreal_test_template");
        let local_repo = git_clone(remote, clone_dir.to_str().unwrap());
        git_pull_ff(local_repo.to_str().unwrap());
    }
}
