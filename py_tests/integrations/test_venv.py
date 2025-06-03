import os
from angreal.integrations.venv import venv_required, VirtualEnv
import shutil
import sys
import pytest
from pathlib import Path
import tempfile

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_venv_required():
    """
    test venv required good
    """

    @venv_required("__angreal", requirements='flask')
    def test(a, b):
        return a + b

    try:
        assert test(1, 2) == 3
    except:
        raise
    finally:
        shutil.rmtree("__angreal")


def test_init():
    """
    testing creation of an environment
    """
    test_requirements = os.path.join(os.path.dirname(__file__), "test_r.txt")

    assert os.path.exists(test_requirements)
    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix

    this_venv = "__test_venv_1"
    assert not os.path.isdir(this_venv)

    VirtualEnv(path=this_venv, requirements=test_requirements,
                now=True).install_requirements()



    try:
        pass
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except Exception:
            pass



def test_requirements_load_string():
    """
    testing load from "string"
    """

    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix

    this_venv = "__test_venv_2"
    assert not os.path.isdir(this_venv)

    VirtualEnv(path=this_venv, requirements="flask", now=True).install_requirements()

    try:
        pass
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except Exception:
            pass


def test_requirements_load_list():
    """
    test load requirements from list
    """
    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix
    this_venv = "__test_venv_3"
    assert not os.path.isdir(this_venv)

    VirtualEnv(path=this_venv, requirements=["flask"], now=True).install_requirements()

    try:
        pass
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except Exception:
            pass


def test_discover_available_pythons():
    """
    Test UV's Python discovery functionality
    """
    pythons = VirtualEnv.discover_available_pythons()
    assert isinstance(pythons, list)
    # Should find at least the current Python
    assert len(pythons) > 0

    # Each entry should be a tuple of (version, path)
    for version, path in pythons:
        assert isinstance(version, str)
        assert isinstance(path, str)
        assert len(version) > 0
        assert len(path) > 0


def test_uv_version():
    """
    Test getting UV version
    """
    version = VirtualEnv.version()
    assert isinstance(version, str)
    assert len(version) > 0
    # UV version typically starts with "uv"
    assert "uv" in version.lower()



@pytest.mark.xfail(reason="UV Python downloads not available for all architectures")
def test_ensure_python():
    """
    Test ensuring a Python version is available
    This test checks if UV can discover existing Python installations
    """
    # Get list of available Python installations first
    pythons = VirtualEnv.discover_available_pythons()

    if not pythons:
        pytest.skip("No Python installations discovered by UV")

    # Find an already installed Python (not a download)
    installed_python = None
    for version, path in pythons:
        # Skip download entries
        if "<download" in path:
            continue
        # Look for a stable version (prefer 3.11 or 3.12)
        if "cpython-3.11" in version or "cpython-3.12" in version:
            installed_python = (version, path)
            break

    if not installed_python:
        # If no preferred version, take any installed version
        for version, path in pythons:
            if "<download" not in path:
                installed_python = (version, path)
                break

    if not installed_python:
        pytest.skip("No installed Python versions found (only download options)")

    version_str, expected_path = installed_python

    # Extract version number to test with
    # From "cpython-3.11.12-macos-aarch64-none" extract "3.11"
    if version_str.startswith("cpython-"):
        # Remove cpython- prefix
        version_part = version_str[8:]  # Skip "cpython-"
        # Extract major.minor
        parts = version_part.split('.')
        if len(parts) >= 2:
            test_version = f"{parts[0]}.{parts[1]}"
        else:
            test_version = parts[0]
    else:
        # Fallback - use the full version
        test_version = version_str

    # Test ensure_python with the extracted version
    result_path = VirtualEnv.ensure_python(test_version)

    assert isinstance(result_path, str)
    assert len(result_path) > 0
    # The path should exist
    assert Path(result_path).exists()
    # It should be one of the discovered paths (might be symlink resolved)
    assert any(Path(p).resolve() == Path(result_path).resolve()
               for _, p in pythons if "<download" not in p)


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_basic_activation():
    """
    Test basic activation and deactivation of virtual environment
    """
    # Save original state
    original_prefix = sys.prefix
    original_path = sys.path.copy()

    # Create a test venv
    venv_path = "test_activation_venv"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        # Test activation
        venv.activate()

        # Check that sys.prefix changed
        assert sys.prefix != original_prefix
        assert str(venv.path) in sys.prefix

        # Check that site-packages is in sys.path
        site_packages_found = any('site-packages' in p and
                                  str(venv.path) in p for p in sys.path)
        assert site_packages_found

        # Test deactivation
        venv.deactivate()

        # Check that state is restored
        assert sys.prefix == original_prefix
        assert sys.path == original_path

    finally:
        # Cleanup
        shutil.rmtree(venv_path)


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_context_manager():
    """
    Test using VirtualEnv as a context manager
    """
    original_prefix = sys.prefix
    original_path = sys.path.copy()
    venv_path = "test_context_venv"

    try:
        with VirtualEnv(path=venv_path, now=True) as venv:
            # Inside context - should be activated
            assert sys.prefix != original_prefix
            assert venv._is_activated

            # Check that site-packages is in sys.path
            site_packages_found = any('site-packages' in p and
                                      str(venv.path) in p for p in sys.path)
            assert site_packages_found

        # Outside context - should be deactivated
        assert sys.prefix == original_prefix
        assert sys.path == original_path

    finally:
        # Cleanup
        shutil.rmtree(venv_path)


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_venv_required_with_activation():
    """
    Test venv_required decorator with activation
    """
    original_prefix = sys.prefix
    test_venv_path = "test_decorator_activation_venv"

    @venv_required(test_venv_path, requirements="six")
    def test_function():
        # This should run in an activated venv
        assert sys.prefix != original_prefix
        assert test_venv_path in sys.prefix

        # Should be able to import the installed package
        import six
        return six.__version__

    try:
        # Call the decorated function
        version = test_function()
        assert version is not None

        # After the function, environment should be restored
        assert sys.prefix == original_prefix

    finally:
        # Cleanup
        shutil.rmtree(test_venv_path)


def test_multiple_activations():
    """
    Test that multiple activations don't cause issues
    """
    original_prefix = sys.prefix
    venv_path = "test_multiple_activation_venv"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        # First activation
        venv.activate()
        first_prefix = sys.prefix
        assert sys.prefix != original_prefix

        # Second activation should be a no-op
        venv.activate()
        assert sys.prefix == first_prefix

        # Single deactivation should restore
        venv.deactivate()
        assert sys.prefix == original_prefix

        # Second deactivation should be a no-op
        venv.deactivate()
        assert sys.prefix == original_prefix

    finally:
        # Cleanup
        shutil.rmtree(venv_path)


def test_virtualenv_now_flag():
    """
    Test that VirtualEnv with now=True actually creates the venv immediately
    """
    # Test with now=True - should create immediately
    venv_path = "test_now_true_venv"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        assert venv.exists
        assert venv.path.exists()
        assert venv.python_executable.exists()
    finally:
        shutil.rmtree(venv_path)

    # Test without now=True - should not create until needed
    venv_path2 = "test_now_false_venv"
    venv2 = VirtualEnv(path=venv_path2, now=False)

    try:
        assert not venv2.path.exists()
        assert not venv2.exists
    finally:
        # Clean up if it was created
        if venv2.path.exists():
            shutil.rmtree(venv_path2)


def test_virtualenv_path_resolution():
    """
    Test absolute vs relative paths
    """
    # Test relative path - should be resolved from current working directory
    rel_venv = VirtualEnv(path="relative-venv", now=False)
    assert rel_venv.path == (Path.cwd() / "relative-venv").resolve()

    # Test absolute path - should be resolved (handles symlinks)
    with tempfile.TemporaryDirectory() as tmpdir:
        abs_path = Path(tmpdir) / "absolute-venv"
        abs_venv = VirtualEnv(path=str(abs_path), now=False)
        # Compare resolved paths to handle symlinks
        assert abs_venv.path == abs_path.resolve()

        # Also test with Path object
        abs_venv2 = VirtualEnv(path=abs_path, now=False)
        assert abs_venv2.path == abs_path.resolve()


def test_virtualenv_api_availability():
    """
    Test which methods and properties exist on VirtualEnv
    """
    venv = VirtualEnv(path="test_api_venv", now=True)

    try:
        # Properties that should exist
        assert hasattr(venv, 'path')
        assert hasattr(venv, 'exists')
        assert hasattr(venv, 'python_executable')

        # Methods that should exist
        assert hasattr(venv, 'install_requirements')
        assert hasattr(venv, 'install')
        assert hasattr(venv, 'activate')
        assert hasattr(venv, 'deactivate')

        # Class methods
        assert hasattr(VirtualEnv, 'discover_available_pythons')
        assert hasattr(VirtualEnv, 'ensure_python')
        assert hasattr(VirtualEnv, 'version')

        # Test that exists is a property, not a method
        assert isinstance(venv.exists, bool)

        # Test that python_executable is a property returning Path
        assert isinstance(venv.python_executable, Path)

    finally:
        shutil.rmtree("test_api_venv")


def test_cross_platform_paths():
    """
    Test that paths work correctly on different platforms
    """
    venv = VirtualEnv(path="test_platform_venv", now=True)

    try:
        if sys.platform == "win32":
            # Windows paths
            assert (venv.path / "Scripts").exists()
            assert (venv.path / "Scripts" / "python.exe") == venv.python_executable
        else:
            # Unix paths
            assert (venv.path / "bin").exists()
            assert (venv.path / "bin" / "python") == venv.python_executable

    finally:
        shutil.rmtree("test_platform_venv")


def test_error_messages():
    """
    Test clear error messages for common issues
    """
    # Test using venv before it's created
    venv = VirtualEnv(path="test_error_venv", now=False)

    try:
        with pytest.raises(RuntimeError) as excinfo:
            venv.activate()
        assert "does not exist" in str(excinfo.value)

    finally:
        if venv.path.exists():
            shutil.rmtree("test_error_venv")

    # Test bad requirements format
    with pytest.raises(TypeError) as excinfo:
        venv = VirtualEnv(path="test_bad_req_venv", requirements=123, now=True)
        venv.install_requirements()
    assert "requirements should be" in str(excinfo.value)

    # Cleanup
    if Path("test_bad_req_venv").exists():
        shutil.rmtree("test_bad_req_venv")


def test_cleanup_and_state():
    """
    Test cleanup and state management
    """
    venv = VirtualEnv(path="test_cleanup_venv", now=True)
    assert venv.exists

    # Manual cleanup
    shutil.rmtree(venv.path)

    # VirtualEnv should handle missing directory gracefully
    assert not venv.exists

    # Should be able to recreate by setting now=True again
    venv2 = VirtualEnv(path="test_cleanup_venv", now=True)
    try:
        assert venv2.exists
    finally:
        shutil.rmtree("test_cleanup_venv")


def test_install_method():
    """
    Test the install() method for installing packages
    """
    venv = VirtualEnv(path="test_install_method_venv", now=True)

    try:
        # Test installing a single package as string
        venv.install("six")

        # Test installing multiple packages as list
        venv.install(["requests", "pytest"])

        # Test installing from requirements file
        req_file = Path("test_requirements.txt")
        req_file.write_text("flask==2.3.0\n")
        venv.install(req_file)

        # Cleanup requirements file
        req_file.unlink()

    finally:
        shutil.rmtree("test_install_method_venv")


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_package_import_after_activation():
    """
    Test that packages installed in venv are importable after activation
    """
    venv = VirtualEnv(path="test_import_venv", now=True)

    try:
        # Install a package that we know isn't in the system Python
        venv.install("toml")  # Using toml as it's small and often not pre-installed

        # Before activation, importing should fail (unless it's in system Python)
        try:
            # Remove toml from sys.modules if it's already imported
            if 'toml' in sys.modules:
                del sys.modules['toml']
            import toml  # noqa: F401
            # If this succeeds, toml is in system Python, skip test
            pytest.skip("toml is already available in system Python")
        except ImportError:
            pass  # Expected

        # Activate the venv
        venv.activate()

        # Now importing should work
        try:
            import toml  # noqa: F811
            # Verify it's from our venv
            assert str(venv.path) in toml.__file__
        finally:
            # Clean up the import
            if 'toml' in sys.modules:
                del sys.modules['toml']
            venv.deactivate()

    finally:
        shutil.rmtree("test_import_venv")
