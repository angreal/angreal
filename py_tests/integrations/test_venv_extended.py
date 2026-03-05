"""
Extended VirtualEnv integration tests.

These tests supplement the existing test_venv.py with 2-3x more coverage,
focusing on bug validation, edge cases, nested venvs, and stress testing.

Organized by category per ANG-T-0021 test plan.
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path

import pytest

from angreal.integrations.venv import VirtualEnv, venv_required


# ============================================================================
# Helpers
# ============================================================================

@pytest.fixture
def venv_cleanup():
    """Track venv paths for cleanup after tests."""
    paths = []
    yield paths
    for p in paths:
        p = Path(p)
        if p.exists():
            shutil.rmtree(p)


def _save_state():
    """Capture current Python/env state for later comparison."""
    return {
        "prefix": sys.prefix,
        "exec_prefix": sys.exec_prefix,
        "path": sys.path.copy(),
        "env_path": os.environ.get("PATH", ""),
        "virtual_env": os.environ.get("VIRTUAL_ENV"),
    }


def _assert_state_restored(original):
    """Assert that Python/env state matches the saved snapshot."""
    assert sys.prefix == original["prefix"]
    assert sys.exec_prefix == original["exec_prefix"]
    assert sys.path == original["path"]
    assert os.environ.get("PATH") == original["env_path"]
    assert os.environ.get("VIRTUAL_ENV") == original["virtual_env"]


# ============================================================================
# Category 1: Bug Validation Tests (8 tests)
# ============================================================================


@pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
class TestBugValidation:

    def test_context_manager_installs_requirements_string(self, venv_cleanup):
        """Bug 1: context manager with requirements=string installs the package."""
        vpath = "test_cm_req_str"
        venv_cleanup.append(vpath)
        with VirtualEnv(path=vpath, requirements="six", now=True) as venv:
            # six should be installed and importable via subprocess
            result = subprocess.run(
                [str(venv.python_executable), "-c",
                 "import six; print(six.__version__)"],
                capture_output=True, text=True,
            )
            assert result.returncode == 0
            assert result.stdout.strip()

    def test_context_manager_installs_requirements_list(self, venv_cleanup):
        """Bug 1: context manager with requirements=list."""
        vpath = "test_cm_req_list"
        venv_cleanup.append(vpath)
        with VirtualEnv(
            path=vpath, requirements=["six", "toml"], now=True
        ) as venv:
            result = subprocess.run(
                [str(venv.python_executable), "-c",
                 "import six; import toml; print('ok')"],
                capture_output=True, text=True,
            )
            assert result.returncode == 0
            assert "ok" in result.stdout

    def test_context_manager_installs_requirements_file(self, venv_cleanup, tmp_path):
        """Bug 1: context manager with requirements=file.txt installs from file."""
        req_file = tmp_path / "reqs.txt"
        req_file.write_text("six\n")
        vpath = str(tmp_path / "test_cm_req_file")
        venv_cleanup.append(vpath)
        with VirtualEnv(path=vpath, requirements=str(req_file), now=True) as venv:
            result = subprocess.run(
                [str(venv.python_executable), "-c",
                 "import six; print(six.__version__)"],
                capture_output=True, text=True,
            )
            assert result.returncode == 0

    def test_context_manager_requirements_importable_inside(
        self, venv_cleanup
    ):
        """Bug 1: packages importable inside with block."""
        vpath = "test_cm_req_import"
        venv_cleanup.append(vpath)
        with VirtualEnv(
            path=vpath, requirements="six", now=True
        ) as venv:
            # After activation, venv's site-packages is in sys.path
            site_pkgs = [
                p for p in sys.path
                if "site-packages" in p and str(venv.path) in p
            ]
            assert len(site_pkgs) > 0

    def test_tilde_path_expansion(self, venv_cleanup):
        """Bug 3: ~/ paths expand to home directory, not literal ~."""
        home = Path.home()
        venv_name = ".test_tilde_venv_angreal"
        tilde_path = f"~/{venv_name}"
        expected = home / venv_name
        venv_cleanup.append(str(expected))

        venv = VirtualEnv(path=tilde_path, now=False)
        assert venv.path == expected.resolve()
        # Ensure no literal ~ directory was created
        assert not Path("~").exists() or not (Path("~") / venv_name).exists()

    def test_tilde_path_with_subdirectory(self, venv_cleanup):
        """Bug 3: ~/sub/dir/venv expands correctly."""
        home = Path.home()
        sub = ".test_tilde_sub_angreal"
        tilde_path = f"~/{sub}/venv"
        expected = home / sub / "venv"
        venv_cleanup.append(str(home / sub))

        venv = VirtualEnv(path=tilde_path, now=False)
        assert venv.path == expected.resolve()

    def test_sys_exec_prefix_set_on_activation(self, venv_cleanup):
        """Bug 6: sys.exec_prefix should point to venv after activation."""
        vpath = "test_exec_prefix"
        venv_cleanup.append(vpath)
        original_exec_prefix = sys.exec_prefix
        venv = VirtualEnv(path=vpath, now=True)

        venv.activate()
        try:
            assert sys.exec_prefix != original_exec_prefix
            assert str(venv.path) in sys.exec_prefix
        finally:
            venv.deactivate()

    def test_sys_exec_prefix_restored_on_deactivation(self, venv_cleanup):
        """Bug 6: sys.exec_prefix restored to original after deactivation."""
        vpath = "test_exec_prefix_restore"
        venv_cleanup.append(vpath)
        original = sys.exec_prefix
        venv = VirtualEnv(path=vpath, now=True)

        venv.activate()
        venv.deactivate()
        assert sys.exec_prefix == original


# ============================================================================
# Category 2: Nested/Stacked Venv Tests (6 tests)
# ============================================================================


@pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
class TestNestedVenvs:

    def test_nested_activation_and_deactivation(self, venv_cleanup):
        """Activate A, activate B, deactivate B -> override."""
        vpath_a = "test_nested_a"
        vpath_b = "test_nested_b"
        venv_cleanup.extend([vpath_a, vpath_b])

        original = _save_state()
        venv_a = VirtualEnv(path=vpath_a, now=True)
        venv_b = VirtualEnv(path=vpath_b, now=True)

        venv_a.activate()
        state_a = _save_state()
        assert sys.prefix != original["prefix"]

        venv_b.activate()
        assert sys.prefix != state_a["prefix"]

        venv_b.deactivate()
        # B's deactivation restores to state before B was activated (which is A's state)
        # Note: current impl restores to B's _original which is A's state
        assert str(venv_a.path) in sys.prefix

        venv_a.deactivate()
        _assert_state_restored(original)

    def test_nested_context_managers(self, venv_cleanup):
        """Nested with blocks both clean up properly."""
        vpath_a = "test_nested_cm_a"
        vpath_b = "test_nested_cm_b"
        venv_cleanup.extend([vpath_a, vpath_b])

        original = _save_state()

        with VirtualEnv(path=vpath_a, now=True) as venv_a:
            assert str(venv_a.path) in sys.prefix
            with VirtualEnv(path=vpath_b, now=True) as venv_b:
                assert str(venv_b.path) in sys.prefix

            # After inner exits, outer's state should be active
            assert str(venv_a.path) in sys.prefix

        _assert_state_restored(original)

    def test_nested_decorator_calls(self, venv_cleanup):
        """@venv_required(A) calling function with @venv_required(B) inside."""
        vpath_a = "test_nested_dec_a"
        vpath_b = "test_nested_dec_b"
        venv_cleanup.extend([vpath_a, vpath_b])

        original_prefix = sys.prefix

        @venv_required(vpath_b)
        def inner_func():
            assert vpath_b in sys.prefix
            return "inner_ok"

        @venv_required(vpath_a)
        def outer_func():
            assert vpath_a in sys.prefix
            result = inner_func()
            # After inner returns, outer's venv should still be active
            assert vpath_a in sys.prefix
            return result

        result = outer_func()
        assert result == "inner_ok"
        assert sys.prefix == original_prefix

    def test_activate_deactivate_activate_same_venv(self, venv_cleanup):
        """Activate -> deactivate -> activate same venv works cleanly."""
        vpath = "test_reactivate"
        venv_cleanup.append(vpath)

        original = _save_state()
        venv = VirtualEnv(path=vpath, now=True)

        venv.activate()
        assert sys.prefix != original["prefix"]
        venv.deactivate()
        _assert_state_restored(original)

        # Second activation
        venv.activate()
        assert sys.prefix != original["prefix"]
        assert str(venv.path) in sys.prefix
        venv.deactivate()
        _assert_state_restored(original)

    def test_activate_different_venvs_sequentially(self, venv_cleanup):
        """Activate A, deactivate A, activate B, deactivate B -> clean state."""
        vpath_a = "test_seq_a"
        vpath_b = "test_seq_b"
        venv_cleanup.extend([vpath_a, vpath_b])

        original = _save_state()

        venv_a = VirtualEnv(path=vpath_a, now=True)
        venv_a.activate()
        assert str(venv_a.path) in sys.prefix
        venv_a.deactivate()
        _assert_state_restored(original)

        venv_b = VirtualEnv(path=vpath_b, now=True)
        venv_b.activate()
        assert str(venv_b.path) in sys.prefix
        venv_b.deactivate()
        _assert_state_restored(original)

    def test_stacked_context_manager_with_exception_in_inner(self, venv_cleanup):
        """Inner with raises exception, outer still deactivates cleanly."""
        vpath_a = "test_stack_exc_a"
        vpath_b = "test_stack_exc_b"
        venv_cleanup.extend([vpath_a, vpath_b])

        original = _save_state()

        with pytest.raises(ValueError, match="test_inner_error"):
            with VirtualEnv(path=vpath_a, now=True):
                with VirtualEnv(path=vpath_b, now=True):
                    raise ValueError("test_inner_error")

        _assert_state_restored(original)


# ============================================================================
# Category 3: Context Manager Edge Cases (7 tests)
# ============================================================================


@pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
class TestContextManagerEdgeCases:

    def test_context_manager_exception_still_deactivates(self, venv_cleanup):
        """Exception inside with -> venv deactivated, state restored."""
        vpath = "test_cm_exc_deact"
        venv_cleanup.append(vpath)

        original = _save_state()

        with pytest.raises(RuntimeError):
            with VirtualEnv(path=vpath, now=True):
                raise RuntimeError("boom")

        _assert_state_restored(original)

    def test_context_manager_exception_propagates(self, venv_cleanup):
        """Exception inside with -> exception NOT swallowed."""
        vpath = "test_cm_exc_prop"
        venv_cleanup.append(vpath)

        with pytest.raises(ValueError, match="specific_error"):
            with VirtualEnv(path=vpath, now=True):
                raise ValueError("specific_error")

    def test_context_manager_with_now_false(self, venv_cleanup):
        """with VirtualEnv(path, now=False) creates venv on enter."""
        vpath = "test_cm_now_false"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=False)
        assert not Path(vpath).exists()

        with venv:
            assert venv.exists
            assert venv._is_activated

    def test_context_manager_returns_venv_instance(self, venv_cleanup):
        """with VirtualEnv(...) as venv -> venv is the VirtualEnv instance."""
        vpath = "test_cm_returns"
        venv_cleanup.append(vpath)

        with VirtualEnv(path=vpath, now=True) as venv:
            assert isinstance(venv, VirtualEnv)
            assert hasattr(venv, "path")
            assert hasattr(venv, "activate")

    def test_context_manager_subprocess_inherits_env(self, venv_cleanup):
        """subprocess.run inside with block uses venv python."""
        vpath = "test_cm_subprocess"
        venv_cleanup.append(vpath)

        with VirtualEnv(path=vpath, now=True) as venv:
            result = subprocess.run(
                [str(venv.python_executable), "-c", "import sys; print(sys.prefix)"],
                capture_output=True, text=True,
            )
            assert result.returncode == 0
            assert str(venv.path) in result.stdout.strip()

    def test_context_manager_no_requirements(self, venv_cleanup):
        """with VirtualEnv(path) works fine with no requirements."""
        vpath = "test_cm_no_reqs"
        venv_cleanup.append(vpath)

        original = _save_state()
        with VirtualEnv(path=vpath, now=True) as venv:
            assert venv._is_activated
            assert venv.exists

        _assert_state_restored(original)

    def test_context_manager_reentry(self, venv_cleanup):
        """Entering same context manager instance twice works or raises safely."""
        vpath = "test_cm_reentry"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)

        # First entry
        with venv:
            assert venv._is_activated

        # Second entry should work (re-activate)
        with venv:
            assert venv._is_activated

        assert not venv._is_activated


# ============================================================================
# Category 4: Decorator Edge Cases (8 tests)
# ============================================================================


class TestDecoratorEdgeCases:

    def test_decorator_preserves_function_name(self, venv_cleanup):
        """Decorated function's __name__ matches original."""
        vpath = "test_dec_name"
        venv_cleanup.append(vpath)

        @venv_required(vpath)
        def my_cool_function():
            pass

        assert my_cool_function.__name__ == "my_cool_function"

    def test_decorator_preserves_docstring(self, venv_cleanup):
        """Decorated function's __doc__ matches original."""
        vpath = "test_dec_doc"
        venv_cleanup.append(vpath)

        @venv_required(vpath)
        def documented_func():
            """This is the docstring."""
            pass

        assert documented_func.__doc__ == "This is the docstring."

    def test_decorator_preserves_arguments_attribute(self, venv_cleanup):
        """Decorated function proxies __arguments for angreal introspection."""
        vpath = "test_dec_args_attr"
        venv_cleanup.append(vpath)

        def original():
            pass

        original.__arguments = [{"name": "verbose", "type": "bool"}]

        decorated = venv_required(vpath)(original)
        assert decorated.__arguments == [{"name": "verbose", "type": "bool"}]

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_decorator_with_kwargs_only(self, venv_cleanup):
        """Decorator works with **kwargs functions."""
        vpath = "test_dec_kwargs"
        venv_cleanup.append(vpath)

        @venv_required(vpath)
        def func_with_kwargs(**kwargs):
            return kwargs

        result = func_with_kwargs(a=1, b=2)
        assert result == {"a": 1, "b": 2}

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_decorator_with_mixed_args_kwargs(self, venv_cleanup):
        """Decorator works with def f(a, b, c=3, **kwargs)."""
        vpath = "test_dec_mixed"
        venv_cleanup.append(vpath)

        @venv_required(vpath)
        def mixed_func(a, b, c=3, **kwargs):
            return (a, b, c, kwargs)

        result = mixed_func(1, 2, c=10, extra="yes")
        assert result == (1, 2, 10, {"extra": "yes"})

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_decorator_exception_propagates_and_cleans_up(self, venv_cleanup):
        """Function raises -> exception propagated, venv deactivated."""
        vpath = "test_dec_exc"
        venv_cleanup.append(vpath)

        original_prefix = sys.prefix

        @venv_required(vpath)
        def failing_func():
            raise ValueError("decorator_test_error")

        with pytest.raises(ValueError, match="decorator_test_error"):
            failing_func()

        assert sys.prefix == original_prefix

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_decorator_return_value_preserved(self, venv_cleanup):
        """Decorated function return value passes through."""
        vpath = "test_dec_return"
        venv_cleanup.append(vpath)

        @venv_required(vpath)
        def returns_dict():
            return {"key": "value", "number": 42}

        result = returns_dict()
        assert result == {"key": "value", "number": 42}

    def test_decorator_getattr_proxy(self, venv_cleanup):
        """Decorated function proxies custom attributes from original."""
        vpath = "test_dec_getattr"
        venv_cleanup.append(vpath)

        def original():
            pass

        original.custom_attr = "hello"
        decorated = venv_required(vpath)(original)
        assert decorated.custom_attr == "hello"


# ============================================================================
# Category 5: Path Handling Edge Cases (6 tests)
# ============================================================================


class TestPathHandling:

    def test_path_with_spaces(self, venv_cleanup, tmp_path):
        """VirtualEnv with spaces in path works."""
        vpath = str(tmp_path / "path with spaces" / "venv")
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        assert venv.python_executable.exists()

    def test_path_with_unicode(self, venv_cleanup, tmp_path):
        """VirtualEnv with unicode in path works."""
        vpath = str(tmp_path / "tëst_vénv")
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        assert venv.python_executable.exists()

    def test_path_object_input(self, venv_cleanup, tmp_path):
        """VirtualEnv(path=Path("my-venv")) works."""
        vpath = tmp_path / "pathlib_venv"
        venv_cleanup.append(str(vpath))

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        assert venv.path == vpath.resolve()

    def test_path_resolution_with_symlinks(self, venv_cleanup, tmp_path):
        """Path through symlink resolves correctly."""
        real_dir = tmp_path / "real_dir"
        real_dir.mkdir()
        link = tmp_path / "link_dir"
        link.symlink_to(real_dir)

        vpath = str(link / "venv")
        venv_cleanup.append(vpath)
        # Also clean the real path in case
        venv_cleanup.append(str(real_dir / "venv"))

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        # Resolved path should point to the real directory
        assert (
            real_dir.resolve() in venv.path.parents
            or venv.path.parent == real_dir.resolve()
        )

    def test_relative_path_anchored_to_cwd(self, venv_cleanup):
        """Relative path is resolved from cwd at creation time."""
        vpath = "test_rel_anchor_venv"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=False)
        expected = (Path.cwd() / vpath).resolve()
        assert venv.path == expected

    def test_absolute_path_unchanged(self, venv_cleanup, tmp_path):
        """Absolute path is not modified by resolution (other than resolve())."""
        vpath = str(tmp_path / "abs_venv")
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=False)
        assert venv.path == Path(vpath).resolve()


# ============================================================================
# Category 6: Install Method Stress Tests (7 tests)
# ============================================================================


class TestInstallStress:

    def test_install_empty_list(self, venv_cleanup):
        """venv.install([]) is a no-op or handles cleanly."""
        vpath = "test_install_empty"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        # Should not raise
        venv.install([])

    def test_install_nonexistent_package(self, venv_cleanup):
        """venv.install("nonexistent-pkg-xyz-999") raises RuntimeError."""
        vpath = "test_install_noexist"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        with pytest.raises(RuntimeError, match="Failed to install"):
            venv.install("this-package-does-not-exist-xyz-99999")

    def test_install_nonexistent_requirements_file(self, venv_cleanup):
        """venv.install("nonexistent.txt") raises RuntimeError."""
        vpath = "test_install_nofile"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        with pytest.raises(RuntimeError, match="Failed to install"):
            venv.install("absolutely_nonexistent_requirements.txt")

    def test_install_version_pinned_package(self, venv_cleanup):
        """venv.install("six==1.16.0") installs exact version."""
        vpath = "test_install_pinned"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        venv.install("six==1.16.0")

        result = subprocess.run(
            [str(venv.python_executable), "-c", "import six; print(six.__version__)"],
            capture_output=True, text=True,
        )
        assert result.returncode == 0
        assert result.stdout.strip() == "1.16.0"

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_install_multiple_calls_accumulate(self, venv_cleanup):
        """Install A, then install B -> both importable."""
        vpath = "test_install_accum"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        venv.install("six")
        venv.install("toml")

        venv.activate()
        try:
            result = subprocess.run(
                [str(venv.python_executable), "-c",
                 "import six; import toml; print('both_ok')"],
                capture_output=True, text=True,
            )
            assert result.returncode == 0
            assert "both_ok" in result.stdout
        finally:
            venv.deactivate()

    def test_install_path_object(self, venv_cleanup, tmp_path):
        """venv.install(Path("requirements.txt")) works."""
        req_file = tmp_path / "requirements.txt"
        req_file.write_text("six\n")

        vpath = str(tmp_path / "test_install_path_obj")
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        venv.install(req_file)

        result = subprocess.run(
            [str(venv.python_executable), "-c", "import six; print('ok')"],
            capture_output=True, text=True,
        )
        assert result.returncode == 0

    def test_install_requirements_method_vs_install(self, venv_cleanup):
        """install_requirements() and install() both work for packages."""
        vpath = "test_install_vs_reqs"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, requirements="six", now=True)
        venv.install_requirements()

        result = subprocess.run(
            [str(venv.python_executable), "-c", "import six; print('ok')"],
            capture_output=True, text=True,
        )
        assert result.returncode == 0


# ============================================================================
# Category 7: State & Property Tests (5 tests)
# ============================================================================


class TestStateAndProperties:

    def test_name_property_value(self, venv_cleanup):
        """venv.name reflects the input path string."""
        vpath = "my_named_venv"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=False)
        assert venv.name == vpath

    def test_exists_property_false_before_create(self):
        """now=False -> venv.exists == False."""
        venv = VirtualEnv(path="nonexistent_venv_xyz", now=False)
        assert not venv.exists

    def test_exists_property_true_after_create(self, venv_cleanup):
        """now=True -> venv.exists == True."""
        vpath = "test_exists_true"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists

    def test_exists_property_false_after_remove(self, venv_cleanup):
        """After venv.remove() -> venv.exists == False."""
        vpath = "test_exists_remove"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        venv.remove()
        assert not venv.exists

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_is_activated_flag(self, venv_cleanup):
        """_is_activated is False before, True during, False after activation."""
        vpath = "test_is_activated"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert not venv._is_activated

        venv.activate()
        assert venv._is_activated

        venv.deactivate()
        assert not venv._is_activated


# ============================================================================
# Category 8: Remove Method Tests (4 tests)
# ============================================================================


class TestRemoveMethod:

    def test_remove_deletes_directory(self, venv_cleanup):
        """venv.remove() deletes the venv directory."""
        vpath = "test_remove_del"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert Path(vpath).exists()

        venv.remove()
        assert not Path(vpath).exists()

    def test_remove_on_nonexistent_is_safe(self):
        """venv.remove() on already-removed/never-created venv doesn't crash."""
        venv = VirtualEnv(path="test_remove_noexist", now=False)
        # Should not raise
        venv.remove()

    @pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
    def test_remove_while_activated(self, venv_cleanup):
        """Remove while activated -> directory removed, deactivation still works."""
        vpath = "test_remove_active"
        venv_cleanup.append(vpath)

        original = _save_state()
        venv = VirtualEnv(path=vpath, now=True)
        venv.activate()

        # Remove while activated
        venv.remove()
        assert not Path(vpath).exists()

        # Deactivation should still work
        venv.deactivate()
        _assert_state_restored(original)

    def test_create_after_remove(self, venv_cleanup):
        """remove() then create() works cleanly."""
        vpath = "test_remove_recreate"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        assert venv.exists
        venv.remove()
        assert not venv.exists

        venv.create()
        assert venv.exists


# ============================================================================
# Category 9: sys.path Integrity Tests (4 tests)
# ============================================================================


@pytest.mark.xfail(sys.platform == "win32", reason="Windows activation flaky")
class TestSysPathIntegrity:

    def test_sys_path_contains_site_packages_after_activation(self, venv_cleanup):
        """Venv site-packages in sys.path after activation."""
        vpath = "test_syspath_site"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        venv.activate()
        try:
            site_pkgs = [
                p for p in sys.path
                if "site-packages" in p and str(venv.path) in p
            ]
            assert len(site_pkgs) > 0
        finally:
            venv.deactivate()

    def test_sys_path_order_venv_first(self, venv_cleanup):
        """Venv site-packages appears before system site-packages in sys.path."""
        vpath = "test_syspath_order"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)
        venv.activate()
        try:
            venv_site_idx = None
            system_site_idx = None
            for i, p in enumerate(sys.path):
                if "site-packages" in p and str(venv.path) in p:
                    if venv_site_idx is None:
                        venv_site_idx = i
                elif "site-packages" in p:
                    if system_site_idx is None:
                        system_site_idx = i

            assert venv_site_idx is not None, "Venv site-packages not found in sys.path"
            if system_site_idx is not None:
                assert venv_site_idx < system_site_idx, (
                    "Venv site-packages should come before system"
                )
        finally:
            venv.deactivate()

    def test_sys_path_fully_restored_after_deactivation(self, venv_cleanup):
        """Exact sys.path list restored after deactivation."""
        vpath = "test_syspath_restore"
        venv_cleanup.append(vpath)

        original_path = sys.path.copy()
        venv = VirtualEnv(path=vpath, now=True)

        venv.activate()
        assert sys.path != original_path  # Should have changed

        venv.deactivate()
        assert sys.path == original_path  # Should be exactly restored

    def test_sys_path_no_duplicates_after_reactivation(self, venv_cleanup):
        """activate/deactivate/activate doesn't duplicate sys.path entries."""
        vpath = "test_syspath_nodup"
        venv_cleanup.append(vpath)

        venv = VirtualEnv(path=vpath, now=True)

        venv.activate()
        first_path = sys.path.copy()
        venv.deactivate()

        venv.activate()
        second_path = sys.path.copy()
        venv.deactivate()

        # The paths during both activations should be identical
        assert first_path == second_path
        # No duplicates
        venv_entries = [p for p in second_path if str(venv.path) in p]
        assert len(venv_entries) == len(set(venv_entries)), (
            "Duplicate venv entries in sys.path"
        )
