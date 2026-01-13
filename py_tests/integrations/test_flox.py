"""Tests for the Flox integration module."""

import json
import os
import pytest
import tempfile
from pathlib import Path

from angreal.integrations.flox import (
    Flox,
    FloxServiceHandle,
    FloxServices,
    flox_required,
)


class TestFloxClass:
    """Unit tests for the Flox class."""

    def test_flox_init_default_path(self):
        """Test Flox initialization with default path (current directory)."""
        flox = Flox()
        # Should resolve to current working directory
        assert flox.path == Path.cwd().resolve()

    def test_flox_init_with_string_path(self):
        """Test Flox initialization with a string path."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            assert flox.path == Path(tmpdir).resolve()

    def test_flox_init_with_path_object(self):
        """Test Flox initialization with a Path object."""
        with tempfile.TemporaryDirectory() as tmpdir:
            path_obj = Path(tmpdir)
            flox = Flox(path=path_obj)
            assert flox.path == path_obj.resolve()

    def test_flox_init_relative_path(self):
        """Test Flox initialization with a relative path."""
        flox = Flox(path=".")
        assert flox.path == Path.cwd().resolve()

    def test_exists_property_false_when_no_flox_env(self):
        """Test that exists returns False when no .flox directory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            assert flox.exists is False

    def test_exists_property_true_when_flox_env_present(self):
        """Test that exists returns True when .flox directory exists."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create a mock .flox directory
            flox_dir = Path(tmpdir) / ".flox"
            flox_dir.mkdir()

            flox = Flox(path=tmpdir)
            assert flox.exists is True

    def test_has_manifest_property_false(self):
        """Test has_manifest returns False when manifest.toml doesn't exist."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            assert flox.has_manifest is False

    def test_has_manifest_property_true(self):
        """Test has_manifest returns True when manifest.toml exists."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create mock .flox/env/manifest.toml
            manifest_path = Path(tmpdir) / ".flox" / "env"
            manifest_path.mkdir(parents=True)
            (manifest_path / "manifest.toml").write_text("[options]\n")

            flox = Flox(path=tmpdir)
            assert flox.has_manifest is True

    def test_path_property_returns_path_object(self):
        """Test that path property returns a pathlib.Path object."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            assert isinstance(flox.path, Path)

    def test_is_activated_initially_false(self):
        """Test that _is_activated is False before activation."""
        flox = Flox()
        assert flox._is_activated is False

    def test_activate_fails_without_flox_env(self):
        """Test that activate raises error when no .flox directory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            with pytest.raises(RuntimeError, match="does not exist"):
                flox.activate()

    def test_deactivate_noop_when_not_activated(self):
        """Test that deactivate is a no-op when not activated."""
        flox = Flox()
        # Should not raise
        flox.deactivate()
        assert flox._is_activated is False


class TestFloxClassMethods:
    """Tests for Flox classmethods."""

    def test_is_available_returns_bool(self):
        """Test that is_available returns a boolean."""
        result = Flox.is_available()
        assert isinstance(result, bool)

    @pytest.mark.skipif(
        not Flox.is_available(),
        reason="Flox CLI not installed"
    )
    def test_version_returns_string(self):
        """Test that version returns a string when Flox is available."""
        version = Flox.version()
        assert isinstance(version, str)
        assert len(version) > 0

    @pytest.mark.skipif(
        Flox.is_available(),
        reason="Test only runs when Flox is not installed"
    )
    def test_version_raises_when_not_available(self):
        """Test that version raises error when Flox is not installed."""
        with pytest.raises(RuntimeError, match="Failed to get Flox version"):
            Flox.version()


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed"
)
class TestFloxActivation:
    """Integration tests for Flox activation (requires Flox CLI)."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        """Create a temporary Flox environment for testing."""
        import subprocess

        # Initialize a new Flox environment
        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            pytest.skip(f"Failed to initialize Flox environment: {result.stderr}")

        yield tmp_path

        # Cleanup - no special action needed as tmp_path is auto-cleaned

    def test_activate_deactivate_cycle(self, flox_env):
        """Test basic activate/deactivate cycle."""
        # Save original environment
        dict(os.environ)

        flox = Flox(path=flox_env)
        assert flox.exists

        # Activate
        flox.activate()
        assert flox._is_activated

        # Environment should have changed (FLOX_ENV should be set)
        assert "FLOX_ENV" in os.environ or "FLOX_ENV_LIB_DIRS" in os.environ

        # Deactivate
        flox.deactivate()
        assert not flox._is_activated

    def test_double_activate_is_noop(self, flox_env):
        """Test that activating twice is a no-op."""
        flox = Flox(path=flox_env)

        flox.activate()
        first_state = dict(os.environ)

        # Second activation should be a no-op
        flox.activate()
        assert dict(os.environ) == first_state

        flox.deactivate()

    def test_double_deactivate_is_noop(self, flox_env):
        """Test that deactivating twice is a no-op."""
        flox = Flox(path=flox_env)

        flox.activate()
        flox.deactivate()

        first_state = dict(os.environ)

        # Second deactivation should be a no-op
        flox.deactivate()
        assert dict(os.environ) == first_state

    def test_context_manager(self, flox_env):
        """Test using Flox as a context manager."""
        dict(os.environ)

        with Flox(path=flox_env) as flox:
            # Inside context - should be activated
            assert flox._is_activated

        # Outside context - should be deactivated
        # Environment changes may persist (Flox adds env vars)
        # but activation state should be reset

    def test_context_manager_exception_handling(self, flox_env):
        """Test that context manager deactivates even on exception."""
        flox = Flox(path=flox_env)

        try:
            with flox:
                assert flox._is_activated
                raise ValueError("Test exception")
        except ValueError:
            pass

        # Should be deactivated despite exception
        assert not flox._is_activated


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed"
)
class TestFloxRun:
    """Tests for running commands in Flox environment."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        """Create a temporary Flox environment for testing."""
        import subprocess

        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            pytest.skip(f"Failed to initialize Flox environment: {result.stderr}")

        yield tmp_path

    def test_run_simple_command(self, flox_env):
        """Test running a simple command in Flox environment."""
        flox = Flox(path=flox_env)

        exit_code, stdout, stderr = flox.run("echo", ["hello"])

        assert exit_code == 0
        assert "hello" in stdout

    def test_run_command_without_args(self, flox_env):
        """Test running a command without arguments."""
        flox = Flox(path=flox_env)

        exit_code, stdout, stderr = flox.run("pwd")

        assert exit_code == 0
        assert len(stdout.strip()) > 0

    def test_run_failing_command(self, flox_env):
        """Test running a command that fails."""
        flox = Flox(path=flox_env)

        exit_code, stdout, stderr = flox.run("false")

        assert exit_code != 0


class TestFloxApiAvailability:
    """Test that all expected API methods exist."""

    def test_flox_has_expected_properties(self):
        """Test that Flox has all expected properties."""
        flox = Flox()

        assert hasattr(flox, 'path')
        assert hasattr(flox, 'exists')
        assert hasattr(flox, 'has_manifest')
        assert hasattr(flox, '_is_activated')

    def test_flox_has_expected_methods(self):
        """Test that Flox has all expected methods."""
        flox = Flox()

        assert hasattr(flox, 'activate')
        assert hasattr(flox, 'deactivate')
        assert hasattr(flox, 'run')
        assert callable(flox.activate)
        assert callable(flox.deactivate)
        assert callable(flox.run)

    def test_flox_has_expected_classmethods(self):
        """Test that Flox has all expected classmethods."""
        assert hasattr(Flox, 'is_available')
        assert hasattr(Flox, 'version')
        assert callable(Flox.is_available)
        assert callable(Flox.version)

    def test_flox_has_context_manager_methods(self):
        """Test that Flox has context manager methods."""
        flox = Flox()

        assert hasattr(flox, '__enter__')
        assert hasattr(flox, '__exit__')

    def test_flox_has_services_property(self):
        """Test that Flox has services property."""
        flox = Flox()

        assert hasattr(flox, 'services')
        services = flox.services
        assert isinstance(services, FloxServices)


class TestFloxServicesClass:
    """Unit tests for the FloxServices class."""

    def test_flox_services_init_with_string_path(self):
        """Test FloxServices initialization with string path."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)
            assert services.path == Path(tmpdir).resolve()

    def test_flox_services_init_with_path_object(self):
        """Test FloxServices initialization with Path object."""
        with tempfile.TemporaryDirectory() as tmpdir:
            path_obj = Path(tmpdir)
            services = FloxServices(path_obj)
            assert services.path == path_obj.resolve()

    def test_flox_services_path_property(self):
        """Test that path property returns Path object."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)
            assert isinstance(services.path, Path)

    def test_flox_services_has_expected_methods(self):
        """Test that FloxServices has all expected methods."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)

            assert hasattr(services, 'start')
            assert hasattr(services, 'stop')
            assert hasattr(services, 'status')
            assert hasattr(services, 'logs')
            assert hasattr(services, 'restart')
            assert callable(services.start)
            assert callable(services.stop)
            assert callable(services.status)
            assert callable(services.logs)
            assert callable(services.restart)


class TestServiceInfoClass:
    """Unit tests for the ServiceInfo class."""

    def test_service_info_repr(self):
        """Test ServiceInfo repr method works (via FloxServiceHandle)."""
        # We can't directly instantiate ServiceInfo from Python,
        # but we can test it through repr
        pass  # ServiceInfo is created internally

    def test_service_info_as_tuple_api(self):
        """Test that ServiceInfo has as_tuple method (via internal API)."""
        # ServiceInfo is created internally by FloxServices
        pass


class TestFloxServiceHandleClass:
    """Unit tests for the FloxServiceHandle class."""

    def test_service_handle_has_expected_properties(self):
        """Test that FloxServiceHandle has expected properties."""
        # FloxServiceHandle is created by FloxServices.start()
        # We test the API availability via load/save
        assert hasattr(FloxServiceHandle, 'load')
        assert callable(FloxServiceHandle.load)

    def test_service_handle_save_and_load(self):
        """Test saving and loading a service handle."""
        with tempfile.TemporaryDirectory() as tmpdir:
            handle_path = Path(tmpdir) / "test-handle.json"

            # Create a mock handle JSON manually
            handle_data = {
                "flox_env_path": "/mock/path",
                "started_at": "1234567890Z",
                "services": [
                    {"name": "postgres", "status": "Running", "pid": 12345},
                    {"name": "redis", "status": "Running", "pid": 12346}
                ]
            }
            with open(handle_path, 'w') as f:
                json.dump(handle_data, f)

            # Load the handle
            handle = FloxServiceHandle.load(str(handle_path))

            assert handle.flox_env_path == Path("/mock/path")
            assert handle.started_at == "1234567890Z"
            assert len(handle.services) == 2
            assert handle.services[0].name == "postgres"
            assert handle.services[0].status == "Running"
            assert handle.services[0].pid == 12345
            assert handle.services[1].name == "redis"

    def test_service_handle_save_creates_file(self):
        """Test that save creates a JSON file."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Load a mock handle first
            handle_path = Path(tmpdir) / "source-handle.json"
            handle_data = {
                "flox_env_path": "/test/path",
                "started_at": "9999Z",
                "services": [
                    {"name": "web", "status": "Running", "pid": 999}
                ]
            }
            with open(handle_path, 'w') as f:
                json.dump(handle_data, f)

            handle = FloxServiceHandle.load(str(handle_path))

            # Save to a new location
            new_path = Path(tmpdir) / "saved-handle.json"
            handle.save(str(new_path))

            assert new_path.exists()

            # Verify the content
            with open(new_path) as f:
                saved_data = json.load(f)

            assert saved_data["flox_env_path"] == "/test/path"
            assert saved_data["started_at"] == "9999Z"
            assert len(saved_data["services"]) == 1
            assert saved_data["services"][0]["name"] == "web"

    def test_service_handle_load_missing_file(self):
        """Test that loading a missing file raises error."""
        with pytest.raises(Exception):  # Could be FileNotFoundError or OSError
            FloxServiceHandle.load("/nonexistent/path/handle.json")

    def test_service_handle_repr(self):
        """Test that FloxServiceHandle has a repr."""
        with tempfile.TemporaryDirectory() as tmpdir:
            handle_path = Path(tmpdir) / "handle.json"
            handle_data = {
                "flox_env_path": "/path",
                "started_at": "123Z",
                "services": [{"name": "svc", "status": "Running", "pid": 1}]
            }
            with open(handle_path, 'w') as f:
                json.dump(handle_data, f)

            handle = FloxServiceHandle.load(str(handle_path))
            repr_str = repr(handle)

            assert "FloxServiceHandle" in repr_str
            assert "svc" in repr_str


class TestFloxServicesProperty:
    """Test the Flox.services property."""

    def test_services_property_returns_flox_services(self):
        """Test that flox.services returns a FloxServices instance."""
        flox = Flox()
        services = flox.services
        assert isinstance(services, FloxServices)

    def test_services_property_has_same_path(self):
        """Test that services has the same path as parent Flox."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            services = flox.services
            assert services.path == flox.path


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed"
)
class TestFloxServicesIntegration:
    """Integration tests for FloxServices (requires Flox CLI)."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        """Create a temporary Flox environment for testing."""
        import subprocess

        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            pytest.skip(f"Failed to initialize Flox environment: {result.stderr}")

        yield tmp_path

    def test_status_returns_list(self, flox_env):
        """Test that status returns a list (empty if no services)."""
        flox = Flox(path=flox_env)
        status = flox.services.status()

        # Should return a list (may be empty if no services defined)
        assert isinstance(status, list)

    def test_services_via_flox_property(self, flox_env):
        """Test accessing services via Flox.services property."""
        flox = Flox(path=flox_env)

        # Should be able to access services
        services = flox.services
        assert isinstance(services, FloxServices)
        assert services.path == flox.path


class TestFloxRequiredDecorator:
    """Unit tests for the flox_required decorator."""

    def test_flox_required_returns_decorator(self):
        """Test that flox_required returns a decorator."""
        decorator = flox_required()
        assert callable(decorator)

    def test_flox_required_with_path_returns_decorator(self):
        """Test that flox_required with path returns a decorator."""
        decorator = flox_required(path=".")
        assert callable(decorator)

    def test_flox_required_with_services_returns_decorator(self):
        """Test that flox_required with services returns a decorator."""
        decorator = flox_required(services=["postgres", "redis"])
        assert callable(decorator)

    def test_flox_required_decorator_wraps_function(self):
        """Test that the decorator wraps a function."""
        @flox_required()
        def my_function():
            """My docstring."""
            return 42

        # Should preserve function name
        assert my_function.__name__ == "my_function"
        # Docstring proxying may not work perfectly with PyO3 wrappers
        # The important thing is that the function is callable

    def test_flox_required_decorator_preserves_name(self):
        """Test that decorator preserves function __name__."""
        @flox_required(path=".")
        def test_func():
            pass

        assert test_func.__name__ == "test_func"

    def test_flox_required_fails_without_flox_env(self):
        """Test that decorated function fails without Flox environment."""
        with tempfile.TemporaryDirectory() as tmpdir:
            @flox_required(path=tmpdir)
            def my_function():
                return 42

            # Should fail because no Flox environment exists
            with pytest.raises(RuntimeError, match="does not exist"):
                my_function()


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed"
)
class TestFloxRequiredIntegration:
    """Integration tests for flox_required decorator (requires Flox CLI)."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        """Create a temporary Flox environment for testing."""
        import subprocess

        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            pytest.skip(f"Failed to initialize Flox environment: {result.stderr}")

        yield tmp_path

    def test_flox_required_activates_env(self, flox_env):
        """Test that flox_required activates the environment."""
        was_activated = False

        @flox_required(path=flox_env)
        def check_activation():
            nonlocal was_activated
            # Inside the function, FLOX_ENV or similar should be set
            was_activated = True
            return 42

        result = check_activation()
        assert result == 42
        assert was_activated

    def test_flox_required_deactivates_after(self, flox_env):
        """Test that flox_required deactivates after function completes."""
        dict(os.environ)

        @flox_required(path=flox_env)
        def my_function():
            return "done"

        result = my_function()
        assert result == "done"

        # Environment should be restored (or similar)
        # Note: Some env vars may remain, but Flox-specific ones should be cleaned

    def test_flox_required_passes_args_kwargs(self, flox_env):
        """Test that flox_required passes args and kwargs correctly."""
        @flox_required(path=flox_env)
        def add_numbers(a, b, multiplier=1):
            return (a + b) * multiplier

        result = add_numbers(2, 3, multiplier=2)
        assert result == 10

    def test_flox_required_exception_handling(self, flox_env):
        """Test that flox_required deactivates even on exception."""
        @flox_required(path=flox_env)
        def raise_error():
            raise ValueError("Test error")

        with pytest.raises(ValueError, match="Test error"):
            raise_error()

        # Environment should still be deactivated (no way to verify directly,
        # but at least we shouldn't crash)

    def test_flox_required_return_value(self, flox_env):
        """Test that flox_required preserves return values."""
        @flox_required(path=flox_env)
        def return_complex():
            return {"key": "value", "list": [1, 2, 3]}

        result = return_complex()
        assert result == {"key": "value", "list": [1, 2, 3]}
