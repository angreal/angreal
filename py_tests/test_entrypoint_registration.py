"""Tests for entrypoint registration functionality."""
import os
import tempfile
import shutil
from pathlib import Path
import pytest
import angreal


class TestEntrypointRegistration:
    """Test the entrypoint registration functions."""

    def setup_method(self):
        """Set up test environment with temporary home directory."""
        self.temp_home = tempfile.mkdtemp()
        self.original_home = os.environ.get('HOME')
        self.original_userprofile = os.environ.get('USERPROFILE')

        # Set both HOME and USERPROFILE for cross-platform compatibility
        os.environ['HOME'] = self.temp_home
        if os.name == 'nt':  # Windows
            os.environ['USERPROFILE'] = self.temp_home

        # Clean up any existing entrypoints to ensure test isolation
        try:
            angreal.cleanup_entrypoints()
        except Exception:
            pass  # Ignore errors during cleanup

    def teardown_method(self):
        """Clean up test environment."""
        # Clean up any entrypoints created during the test
        try:
            angreal.cleanup_entrypoints()
        except Exception:
            pass  # Ignore errors during cleanup

        # Restore original environment
        if self.original_home:
            os.environ['HOME'] = self.original_home
        else:
            os.environ.pop('HOME', None)

        if self.original_userprofile:
            os.environ['USERPROFILE'] = self.original_userprofile
        elif os.name == 'nt':  # Windows
            os.environ.pop('USERPROFILE', None)

        shutil.rmtree(self.temp_home, ignore_errors=True)

    def test_register_entrypoint(self):
        """Test basic entrypoint registration."""
        alias_name = "test-tool"

        # Register the entrypoint
        angreal.register_entrypoint(alias_name)

        # Check that script was created (platform-specific path)
        if os.name == 'nt':  # Windows
            script_path = Path(self.temp_home) / ".local" / "bin" / f"{alias_name}.bat"
        else:  # Unix
            script_path = Path(self.temp_home) / ".local" / "bin" / alias_name

        assert script_path.exists()
        assert script_path.is_file()

        # Check script content
        content = script_path.read_text()
        assert "ANGREAL_ALIAS:" in content
        assert alias_name in content

        if os.name == 'nt':  # Windows
            assert "python -m angreal" in content
        else:  # Unix
            assert "angreal.main()" in content

        # Check that registry was updated
        aliases = angreal.list_entrypoints()
        assert alias_name in aliases

    def test_list_entrypoints(self):
        """Test listing registered entrypoints."""
        # Initially empty
        aliases = angreal.list_entrypoints()
        assert aliases == []

        # Register some entrypoints
        angreal.register_entrypoint("tool1")
        angreal.register_entrypoint("tool2")

        aliases = angreal.list_entrypoints()
        assert len(aliases) == 2
        assert "tool1" in aliases
        assert "tool2" in aliases

    def test_unregister_entrypoint(self):
        """Test unregistering an entrypoint."""
        alias_name = "temp-tool"

        # Register then unregister
        angreal.register_entrypoint(alias_name)
        assert alias_name in angreal.list_entrypoints()

        angreal.unregister_entrypoint(alias_name)

        # Check script was removed (platform-specific path)
        if os.name == 'nt':  # Windows
            script_path = Path(self.temp_home) / ".local" / "bin" / f"{alias_name}.bat"
        else:  # Unix
            script_path = Path(self.temp_home) / ".local" / "bin" / alias_name
        assert not script_path.exists()

        # Check registry was updated
        aliases = angreal.list_entrypoints()
        assert alias_name not in aliases

    def test_cleanup_entrypoints(self):
        """Test cleaning up all entrypoints."""
        # Register multiple entrypoints
        angreal.register_entrypoint("cleanup1")
        angreal.register_entrypoint("cleanup2")
        angreal.register_entrypoint("cleanup3")

        assert len(angreal.list_entrypoints()) == 3

        # Cleanup all
        angreal.cleanup_entrypoints()

        # Check all are removed
        aliases = angreal.list_entrypoints()
        assert aliases == []

        # Check scripts are removed (platform-specific paths)
        bin_dir = Path(self.temp_home) / ".local" / "bin"
        for name in ["cleanup1", "cleanup2", "cleanup3"]:
            if os.name == 'nt':  # Windows
                assert not (bin_dir / f"{name}.bat").exists()
            else:  # Unix
                assert not (bin_dir / name).exists()

    def test_register_duplicate_fails(self):
        """Test that registering a duplicate entrypoint fails."""
        alias_name = "duplicate-tool"

        # Create a conflicting file (platform-specific path)
        if os.name == 'nt':  # Windows
            script_path = Path(self.temp_home) / ".local" / "bin" / f"{alias_name}.bat"
        else:  # Unix
            script_path = Path(self.temp_home) / ".local" / "bin" / alias_name

        script_path.parent.mkdir(parents=True, exist_ok=True)
        script_path.write_text("existing script")

        # Should fail to register
        with pytest.raises(RuntimeError, match="already exists"):
            angreal.register_entrypoint(alias_name)

    def test_script_permissions_unix(self):
        """Test that created scripts have executable permissions on Unix."""
        import stat

        # Skip this test on Windows since executable permissions work differently
        if os.name == 'nt':
            pytest.skip("Executable permissions test not applicable on Windows")

        alias_name = "perm-test"
        angreal.register_entrypoint(alias_name)

        script_path = Path(self.temp_home) / ".local" / "bin" / alias_name
        file_stat = script_path.stat()

        # Check that owner has execute permission
        assert file_stat.st_mode & stat.S_IXUSR

    def test_script_functionality(self):
        """Test that the generated script would work correctly."""
        alias_name = "func-test"
        angreal.register_entrypoint(alias_name)

        # Get platform-specific script path
        if os.name == 'nt':  # Windows
            script_path = Path(self.temp_home) / ".local" / "bin" / f"{alias_name}.bat"
        else:  # Unix
            script_path = Path(self.temp_home) / ".local" / "bin" / alias_name

        content = script_path.read_text()

        if os.name == 'nt':  # Windows
            # Windows batch script should contain python command
            assert "python -m angreal" in content
            assert "ANGREAL_ALIAS:" in content
        else:  # Unix
            # Unix script should be a valid Python script
            assert content.startswith("#!/usr/bin/env python3")
            assert "import sys" in content
            assert "import angreal" in content
            assert "angreal.main()" in content
            # Should have error handling for missing angreal
            assert "ImportError" in content
            assert "angreal not installed" in content

    def test_registry_persistence(self):
        """Test that the registry persists between function calls."""
        # Register entrypoint
        angreal.register_entrypoint("persist-test")

        # Simulate a fresh start by creating a new instance
        aliases1 = angreal.list_entrypoints()
        assert "persist-test" in aliases1

        # Register another
        angreal.register_entrypoint("persist-test2")

        aliases2 = angreal.list_entrypoints()
        assert len(aliases2) == 2
        assert "persist-test" in aliases2
        assert "persist-test2" in aliases2

    def test_unregister_nonexistent(self):
        """Test unregistering a non-existent entrypoint doesn't fail."""
        # Should not raise an error
        angreal.unregister_entrypoint("does-not-exist")

        # Registry should still work
        aliases = angreal.list_entrypoints()
        assert aliases == []
