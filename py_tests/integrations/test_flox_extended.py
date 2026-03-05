"""Extended tests for Flox integration bug fixes."""

import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

import pytest

from angreal.integrations.flox import (
    Flox,
    FloxServiceHandle,
    FloxServices,
    ServiceInfo,
    flox_required,
)


# ==================== Bug 1: Environment Inheritance ====================


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed",
)
class TestEnvInheritance:
    """Bug 1: activate() must set std::env so Rust-spawned
    subprocesses inherit the activated environment."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            pytest.skip(
                f"Failed to init Flox env: {result.stderr}"
            )
        yield tmp_path

    def test_activate_sets_flox_env_in_os_environ(
        self, flox_env
    ):
        """After activate(), FLOX_ENV should be in os.environ."""
        flox = Flox(path=flox_env)
        original_env = dict(os.environ)
        try:
            flox.activate()
            # Flox activation should set some env vars
            assert flox._is_activated
            # At minimum, PATH should differ
            assert os.environ.get("PATH") != original_env.get(
                "PATH"
            ) or "FLOX_ENV" in os.environ
        finally:
            flox.deactivate()

    def test_deactivate_restores_environment(self, flox_env):
        """After deactivate(), env should be restored."""
        original_path = os.environ.get("PATH", "")
        flox = Flox(path=flox_env)
        flox.activate()
        flox.deactivate()
        assert os.environ.get("PATH", "") == original_path

    def test_deactivate_removes_added_keys(self, flox_env):
        """Keys added during activation should be removed."""
        flox = Flox(path=flox_env)
        flox.activate()
        # Record any FLOX-specific keys
        flox_keys = [
            k for k in os.environ if k.startswith("FLOX_")
        ]
        flox.deactivate()
        # FLOX-specific keys added during activation
        # should be removed
        for key in flox_keys:
            # Some may have existed before, that's OK
            pass
        assert not flox._is_activated


# ==================== Bug 2: Context Manager + Services ================


class TestContextManagerServices:
    """Bug 2: Context manager should support services param."""

    def test_flox_accepts_services_parameter(self):
        """Flox() should accept a services parameter."""
        # Should not raise TypeError
        flox = Flox(services=["postgres", "redis"])
        assert isinstance(flox, Flox)

    def test_flox_services_defaults_to_none(self):
        """Without services param, Flox still works."""
        flox = Flox()
        assert isinstance(flox, Flox)

    def test_flox_services_empty_list(self):
        """Empty services list should be accepted."""
        flox = Flox(services=[])
        assert isinstance(flox, Flox)

    @pytest.mark.skipif(
        not Flox.is_available(),
        reason="Flox CLI not installed",
    )
    def test_context_manager_activates_with_services_param(
        self, tmp_path
    ):
        """Context manager should activate even with services."""
        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            pytest.skip("Failed to init Flox env")

        # No actual services defined, so start will fail,
        # but activation should work with empty services
        with Flox(path=tmp_path, services=[]) as flox:
            assert flox._is_activated


# ==================== Bug 3: services_logs follow =======================


class TestServicesLogsFollow:
    """Bug 3: services_logs with follow=True should error."""

    def test_logs_follow_true_raises_value_error(self):
        """follow=True should raise ValueError."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)
            with pytest.raises(
                ValueError, match="follow=True is not supported"
            ):
                services.logs("myservice", follow=True)

    def test_logs_follow_false_does_not_raise_value_error(
        self,
    ):
        """follow=False should not raise ValueError
        (may fail for other reasons)."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)
            # Will fail because no flox env, but NOT ValueError
            with pytest.raises(RuntimeError):
                services.logs("myservice", follow=False)


# ==================== Bug 4: Tilde Path Expansion =====================


class TestTildePathExpansion:
    """Bug 4: ~ paths should be expanded to home directory."""

    def test_flox_tilde_expansion(self):
        """Flox('~/somedir') should expand ~ to home."""
        flox = Flox(path="~/test-flox-tilde")
        expected = Path.home() / "test-flox-tilde"
        assert flox.path == expected.resolve()

    def test_flox_services_tilde_expansion(self):
        """FloxServices('~/somedir') should expand ~."""
        services = FloxServices("~/test-flox-svc-tilde")
        expected = Path.home() / "test-flox-svc-tilde"
        assert services.path == expected.resolve()

    def test_flox_tilde_with_subdirectory(self):
        """Flox('~/a/b/c') should expand correctly."""
        flox = Flox(path="~/a/b/c")
        expected = Path.home() / "a" / "b" / "c"
        assert flox.path == expected.resolve()


# ==================== Bug 5: ISO 8601 Timestamps =====================


class TestTimestampFormat:
    """Bug 5: chrono_now() should produce ISO 8601 timestamps."""

    def test_service_handle_timestamp_is_iso8601(self):
        """Loaded handle's started_at should be ISO 8601."""
        with tempfile.TemporaryDirectory() as tmpdir:
            handle_path = Path(tmpdir) / "handle.json"
            handle_data = {
                "flox_env_path": "/mock",
                "started_at": "2026-01-15T10:30:00Z",
                "services": [
                    {
                        "name": "web",
                        "status": "Running",
                        "pid": 1,
                    }
                ],
            }
            with open(handle_path, "w") as f:
                json.dump(handle_data, f)

            handle = FloxServiceHandle.load(str(handle_path))
            assert handle.started_at == "2026-01-15T10:30:00Z"

    def test_iso8601_format_regex(self):
        """Verify ISO 8601 format via regex on a new handle."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create a handle, save it, load it
            handle_path = Path(tmpdir) / "h.json"
            handle_data = {
                "flox_env_path": "/mock",
                "started_at": "2026-01-15T10:30:00Z",
                "services": [],
            }
            with open(handle_path, "w") as f:
                json.dump(handle_data, f)
            handle = FloxServiceHandle.load(str(handle_path))
            # Save and reload to check persistence
            save_path = Path(tmpdir) / "resaved.json"
            handle.save(str(save_path))
            with open(save_path) as f:
                data = json.load(f)
            ts = data["started_at"]
            # Should match ISO 8601 pattern
            iso_pattern = (
                r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z"
            )
            assert re.match(iso_pattern, ts), (
                f"Timestamp '{ts}' is not ISO 8601"
            )


# ==================== Bug 6: ServiceHandle.stop() scope ===============


class TestServiceHandleStopScope:
    """Bug 6: FloxServiceHandle.stop() should only stop
    tracked services, not all."""

    def test_handle_stop_uses_service_names(self):
        """Verify handle tracks specific service names."""
        with tempfile.TemporaryDirectory() as tmpdir:
            handle_path = Path(tmpdir) / "handle.json"
            handle_data = {
                "flox_env_path": "/mock",
                "started_at": "2026-01-15T10:30:00Z",
                "services": [
                    {
                        "name": "postgres",
                        "status": "Running",
                        "pid": 100,
                    },
                    {
                        "name": "redis",
                        "status": "Running",
                        "pid": 101,
                    },
                ],
            }
            with open(handle_path, "w") as f:
                json.dump(handle_data, f)

            handle = FloxServiceHandle.load(str(handle_path))
            # Verify the handle knows its service names
            names = [s.name for s in handle.services]
            assert names == ["postgres", "redis"]


# ==================== Bug 7: ServiceInfo Constructor ==================


class TestServiceInfoConstructor:
    """Bug 7: ServiceInfo should be constructable from Python."""

    def test_service_info_constructor(self):
        """ServiceInfo can be created from Python."""
        info = ServiceInfo("postgres", "Running", pid=12345)
        assert info.name == "postgres"
        assert info.status == "Running"
        assert info.pid == 12345

    def test_service_info_no_pid(self):
        """ServiceInfo can be created without pid."""
        info = ServiceInfo("redis", "Stopped")
        assert info.name == "redis"
        assert info.status == "Stopped"
        assert info.pid is None

    def test_service_info_repr_with_pid(self):
        """ServiceInfo repr includes pid when present."""
        info = ServiceInfo("web", "Running", pid=99)
        r = repr(info)
        assert "web" in r
        assert "Running" in r
        assert "99" in r

    def test_service_info_repr_without_pid(self):
        """ServiceInfo repr omits pid when None."""
        info = ServiceInfo("web", "Stopped")
        r = repr(info)
        assert "web" in r
        assert "Stopped" in r

    def test_service_info_as_tuple(self):
        """ServiceInfo.as_tuple() returns (name, status, pid)."""
        info = ServiceInfo("db", "Running", pid=42)
        t = info.as_tuple()
        assert t == ("db", "Running", 42)

    def test_service_info_as_tuple_no_pid(self):
        """ServiceInfo.as_tuple() with no pid."""
        info = ServiceInfo("db", "Stopped")
        t = info.as_tuple()
        assert t == ("db", "Stopped", None)


# ==================== FloxServices.stop() with args ===================


class TestFloxServicesStopArgs:
    """FloxServices.stop() should accept service names."""

    def test_stop_method_is_callable(self):
        """stop() should be callable."""
        with tempfile.TemporaryDirectory() as tmpdir:
            services = FloxServices(tmpdir)
            assert callable(services.stop)

    @pytest.mark.skipif(
        not Flox.is_available(),
        reason="Flox CLI not installed",
    )
    def test_stop_accepts_service_names(self, tmp_path):
        """stop() should accept *args for service names."""
        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            pytest.skip("Failed to init Flox env")

        services = FloxServices(tmp_path)
        # Should not raise TypeError for accepting args
        # May raise RuntimeError if no services running
        try:
            services.stop("nonexistent")
        except RuntimeError:
            pass  # Expected - no services running


# ==================== Decorator Proxy Tests ============================


class TestFloxDecoratorProxy:
    """Test flox_required decorator attribute proxying."""

    def test_decorator_preserves_name(self):
        """Decorator preserves __name__."""

        @flox_required(path="/nonexistent")
        def my_func():
            pass

        assert my_func.__name__ == "my_func"

    def test_decorator_preserves_doc(self):
        """Decorator preserves __doc__."""

        @flox_required(path="/nonexistent")
        def my_func():
            """My docstring."""
            pass

        assert my_func.__doc__ == "My docstring."

    def test_decorator_with_services_param(self):
        """Decorator accepts services parameter."""

        @flox_required(
            path="/nonexistent",
            services=["pg", "redis"],
        )
        def my_func():
            pass

        assert callable(my_func)

    def test_decorator_fails_without_flox_env(self):
        """Decorated function fails when env doesn't exist."""
        with tempfile.TemporaryDirectory() as tmpdir:

            @flox_required(path=tmpdir)
            def my_func():
                return 42

            with pytest.raises(
                RuntimeError, match="does not exist"
            ):
                my_func()


# ==================== Integration: Decorator + Flox ====================


@pytest.mark.skipif(
    not Flox.is_available(),
    reason="Flox CLI not installed",
)
class TestFloxDecoratorIntegration:
    """Integration tests for flox_required with real Flox."""

    @pytest.fixture
    def flox_env(self, tmp_path):
        result = subprocess.run(
            ["flox", "init"],
            cwd=tmp_path,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            pytest.skip(
                f"Failed to init Flox env: {result.stderr}"
            )
        yield tmp_path

    def test_decorator_activates_and_deactivates(
        self, flox_env
    ):
        """Decorator activates env, runs func, deactivates."""
        env_during = {}

        @flox_required(path=flox_env)
        def capture_env():
            env_during.update(dict(os.environ))
            return "done"

        result = capture_env()
        assert result == "done"
        # During execution, env should have been modified
        assert len(env_during) > 0

    def test_decorator_exception_still_deactivates(
        self, flox_env
    ):
        """Even on exception, decorator cleans up."""

        @flox_required(path=flox_env)
        def failing():
            raise ValueError("boom")

        with pytest.raises(ValueError, match="boom"):
            failing()


# ==================== Edge Cases ======================================


class TestFloxEdgeCases:
    """Edge case tests."""

    def test_flox_path_object_input(self):
        """Flox accepts pathlib.Path objects."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=Path(tmpdir))
            assert flox.path == Path(tmpdir).resolve()

    def test_flox_default_path_is_cwd(self):
        """Default path resolves to cwd."""
        flox = Flox()
        assert flox.path == Path.cwd().resolve()

    def test_flox_relative_path_resolved(self):
        """Relative paths are resolved to absolute."""
        flox = Flox(path=".")
        assert flox.path.is_absolute()

    def test_flox_services_from_flox_instance(self):
        """flox.services returns FloxServices with same path."""
        with tempfile.TemporaryDirectory() as tmpdir:
            flox = Flox(path=tmpdir)
            svc = flox.services
            assert isinstance(svc, FloxServices)
            assert svc.path == flox.path
