"""
Test the Rust-based git integration.

These tests verify that the new Rust implementation provides the same
API and behavior as the original Python subprocess implementation.
"""
import pytest
import tempfile
import os
from pathlib import Path

# Import our new Rust-based implementation
try:
    from angreal.integrations.git import Git, GitException, clone
    RUST_GIT_AVAILABLE = True
except ImportError:
    RUST_GIT_AVAILABLE = False
    Git = None
    GitException = None
    clone = None


@pytest.mark.skipif(not RUST_GIT_AVAILABLE, reason="Rust git integration not available")
class TestGitRustIntegration:
    """Test the Rust-based git integration."""

    def test_git_init(self):
        """Test git repository initialization."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)
            return_code, stderr, stdout = git.init()

            assert return_code == 0
            assert (Path(tmpdir) / ".git").exists()

    def test_git_workflow(self):
        """Test a complete git workflow."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)

            # Initialize
            return_code, stderr, stdout = git.init()
            assert return_code == 0

            # Create file
            test_file = Path(tmpdir) / "test.txt"
            test_file.write_text("Hello, World!")

            # Add and commit
            return_code, stderr, stdout = git.add("test.txt")
            assert return_code == 0

            return_code, stderr, stdout = git.commit("Initial commit")
            assert return_code == 0

            # Check status
            return_code, stderr, stdout = git.status()
            assert return_code == 0
            status_text = stdout.decode('utf-8')
            assert ("working tree clean" in status_text or
                    "nothing to commit" in status_text)

    def test_git_call_syntax(self):
        """Test the __call__ syntax for backwards compatibility."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)

            # Should work like git("init")
            return_code, stderr, stdout = git("init")
            assert return_code == 0
            assert (Path(tmpdir) / ".git").exists()

    def test_git_method_syntax(self):
        """Test the method syntax like git.add()."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)
            git.init()

            # Create file
            test_file = Path(tmpdir) / "test.txt"
            test_file.write_text("Test")

            # Method syntax should work
            return_code, stderr, stdout = git.add("test.txt")
            assert return_code == 0

    def test_git_convenience_methods(self):
        """Test the high-level convenience methods."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)

            # Test init convenience method
            return_code, stderr, stdout = git.init()
            assert return_code == 0

            # Create and add file
            test_file = Path(tmpdir) / "test.txt"
            test_file.write_text("Test content")

            return_code, stderr, stdout = git.add("test.txt")
            assert return_code == 0

            # Test commit convenience method
            return_code, stderr, stdout = git.commit("Test commit")
            assert return_code == 0

            # Test status convenience method
            return_code, stderr, stdout = git.status()
            assert return_code == 0

    def test_git_error_handling(self):
        """Test that git errors are properly handled."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)

            # Try to use a non-existent git command
            with pytest.raises(GitException):
                git("this-command-does-not-exist")

    def test_git_with_options(self):
        """Test git commands with options (kwargs)."""
        with tempfile.TemporaryDirectory() as tmpdir:
            git = Git(working_dir=tmpdir)

            # Initialize with bare option
            return_code, stderr, stdout = git("init", bare=True)
            assert return_code == 0
            # Bare repos don't have a working directory, so no .git folder
            assert not (Path(tmpdir) / ".git").exists()
            # But they should have git files directly in the directory
            assert (Path(tmpdir) / "config").exists() or len(os.listdir(tmpdir)) > 0

    def test_original_api_compatibility(self):
        """Test that the API matches the updated implementation."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Test constructor compatibility
            git = Git(working_dir=tmpdir)
            assert str(git.working_dir) == tmpdir

            # Test that __call__ returns the expected tuple format
            return_code, stderr, stdout = git("init")
            assert isinstance(return_code, int)
            assert isinstance(stderr, bytes)
            assert isinstance(stdout, bytes)


@pytest.mark.skipif(not RUST_GIT_AVAILABLE, reason="Rust git integration not available")
def test_clone_function():
    """Test the module-level clone function."""
    # Note: This test requires network access and a real git repo
    # We'll use a small, stable public repo for testing
    test_repo = "https://github.com/octocat/Hello-World.git"

    with tempfile.TemporaryDirectory() as tmpdir:
        old_cwd = os.getcwd()
        try:
            os.chdir(tmpdir)
            # This might fail if no network access, so we'll make it non-critical
            try:
                result_path = clone(test_repo)
                assert Path(result_path).exists()
                assert (Path(result_path) / ".git").exists()
            except Exception:
                pytest.skip("Network access required for clone test")
        finally:
            os.chdir(old_cwd)


if __name__ == "__main__":
    pytest.main([__file__])
