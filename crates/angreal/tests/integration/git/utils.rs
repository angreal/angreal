use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A test fixture that manages temporary directories and working directory state
/// for git-related tests.
pub struct GitTestFixture {
    temp_dir: TempDir,
    original_dir: PathBuf,
}

impl GitTestFixture {
    /// Creates a new test fixture with a unique temporary directory
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let original_dir = std::env::current_dir().expect("Failed to get current directory");

        Self {
            temp_dir,
            original_dir,
        }
    }

    /// Returns the path to the temporary directory
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Changes the working directory to the temporary directory
    pub fn change_to_temp_dir(&self) {
        std::env::set_current_dir(self.temp_path())
            .expect("Failed to change to temporary directory");
    }

    /// Creates a subdirectory in the temporary directory
    pub fn create_subdir(&self, name: &str) -> PathBuf {
        let path = self.temp_path().join(name);
        std::fs::create_dir(&path).expect("Failed to create subdirectory");
        path
    }

    /// Cleans up the temporary directory
    pub fn cleanup(&self) {
        if let Err(e) = std::fs::remove_dir_all(self.temp_path()) {
            eprintln!("Warning: Failed to clean up temporary directory: {}", e);
        }
    }
}

impl Drop for GitTestFixture {
    fn drop(&mut self) {
        // Restore original working directory
        if let Err(e) = std::env::set_current_dir(&self.original_dir) {
            eprintln!("Warning: Failed to restore original directory: {}", e);
        }
        self.cleanup();
    }
}
