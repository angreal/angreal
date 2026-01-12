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


def test_install_failure_scenarios():
    """
    Test that install fails gracefully in error scenarios
    """
    # Test 1: Install on broken venv (missing bin directory)
    venv = VirtualEnv(path="test_broken_venv", now=True)

    try:
        # Break the venv by removing bin/Scripts directory
        import os
        if os.name == 'nt':  # Windows
            bin_dir = venv.path / "Scripts"
        else:  # Unix
            bin_dir = venv.path / "bin"
        if bin_dir.exists():
            shutil.rmtree(bin_dir)

        # Install should fail gracefully
        with pytest.raises(RuntimeError, match="Failed to install"):
            venv.install("six")

    finally:
        if Path("test_broken_venv").exists():
            shutil.rmtree("test_broken_venv")

    # Test 2: Install on nonexistent venv
    venv2 = VirtualEnv(path="test_nonexistent_venv", now=False)  # Don't create

    try:
        # Install should fail gracefully
        with pytest.raises(RuntimeError, match="Failed to install"):
            venv2.install("six")

    finally:
        if Path("test_nonexistent_venv").exists():
            shutil.rmtree("test_nonexistent_venv")


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_environment_variables_set():
    """
    Test that activation sets PATH and VIRTUAL_ENV environment variables
    """
    original_path = os.environ.get("PATH", "")
    original_virtual_env = os.environ.get("VIRTUAL_ENV")

    venv_path = "test_env_vars_venv"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        # Activate the venv
        venv.activate()

        # Check that PATH was modified
        new_path = os.environ.get("PATH", "")
        assert new_path != original_path

        # Check that venv's bin directory is in PATH
        if sys.platform == "win32":
            bin_dir = str(venv.path / "Scripts")
        else:
            bin_dir = str(venv.path / "bin")
        assert bin_dir in new_path

        # Check that bin dir is at the beginning of PATH
        assert new_path.startswith(bin_dir)

        # Check that VIRTUAL_ENV was set
        assert os.environ.get("VIRTUAL_ENV") == str(venv.path)

        # Deactivate
        venv.deactivate()

        # Check that PATH was restored
        assert os.environ.get("PATH") == original_path

        # Check that VIRTUAL_ENV was restored/removed
        assert os.environ.get("VIRTUAL_ENV") == original_virtual_env

    finally:
        # Cleanup
        shutil.rmtree(venv_path)


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_subprocess_uses_venv_python():
    """
    Test that subprocess calls to 'python' use the venv's Python, not system Python
    """
    import subprocess

    # Get system Python path
    system_python = sys.executable

    venv_path = "test_subprocess_venv"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        # Before activation, subprocess should use system Python
        result = subprocess.run(
            ["python", "-c", "import sys; print(sys.executable)"],
            capture_output=True,
            text=True
        )
        before_python = result.stdout.strip()

        # Activate the venv
        venv.activate()

        # After activation, subprocess should use venv's Python
        result = subprocess.run(
            ["python", "-c", "import sys; print(sys.executable)"],
            capture_output=True,
            text=True
        )
        after_python = result.stdout.strip()

        # The venv Python should be different from system Python
        assert after_python != system_python
        assert after_python != before_python

        # The venv Python should be in the venv directory
        assert str(venv.path) in after_python

        # Deactivate
        venv.deactivate()

        # After deactivation, should go back to system Python
        result = subprocess.run(
            ["python", "-c", "import sys; print(sys.executable)"],
            capture_output=True,
            text=True
        )
        restored_python = result.stdout.strip()
        assert restored_python == before_python

    finally:
        # Cleanup
        shutil.rmtree(venv_path)


@pytest.mark.xfail(sys.platform == 'win32',
                   reason="Virtual environment activation fails on Windows")
def test_venv_required_decorator_subprocess():
    """
    Test that the venv_required decorator allows subprocess to use venv's Python
    """
    import subprocess

    test_venv_path = "test_decorator_subprocess_venv"
    system_python = sys.executable

    @venv_required(test_venv_path, requirements="six")
    def test_function():
        # Inside the decorated function, subprocess should use venv Python
        result = subprocess.run(
            ["python", "-c", "import sys; print(sys.executable)"],
            capture_output=True,
            text=True
        )
        venv_python = result.stdout.strip()

        # Should not be system Python
        assert venv_python != system_python

        # Should be in the venv directory
        assert test_venv_path in venv_python

        # Verify we can import the installed package via subprocess
        result = subprocess.run(
            ["python", "-c", "import six; print(six.__version__)"],
            capture_output=True,
            text=True
        )
        assert result.returncode == 0
        assert result.stdout.strip()  # Should print version

        return venv_python

    try:
        # Call the decorated function
        venv_python = test_function()
        assert venv_python is not None

        # After the function, subprocess should use system Python again
        result = subprocess.run(
            ["python", "-c", "import sys; print(sys.executable)"],
            capture_output=True,
            text=True
        )
        restored_python = result.stdout.strip()

        # Should be back to something that's not the venv
        assert test_venv_path not in restored_python

    finally:
        # Cleanup
        shutil.rmtree(test_venv_path)


def test_virtual_env_already_set():
    """
    Test that activation handles existing VIRTUAL_ENV correctly
    """
    # Set a fake VIRTUAL_ENV
    original_venv = os.environ.get("VIRTUAL_ENV")
    os.environ["VIRTUAL_ENV"] = "/fake/venv/path"

    venv_path = "test_existing_venv_var"
    venv = VirtualEnv(path=venv_path, now=True)

    try:
        # Activate
        venv.activate()

        # Should be set to our venv
        assert os.environ.get("VIRTUAL_ENV") == str(venv.path)

        # Deactivate
        venv.deactivate()

        # Should be restored to the fake path
        assert os.environ.get("VIRTUAL_ENV") == "/fake/venv/path"

    finally:
        # Cleanup
        shutil.rmtree(venv_path)
        # Restore original state
        if original_venv is None:
            os.environ.pop("VIRTUAL_ENV", None)
        else:
            os.environ["VIRTUAL_ENV"] = original_venv


# ==================== Functional/Integration Tests ====================
# These tests exercise the venv integration through actual angreal task execution


class TestVenvFunctionalIntegration:
    """
    Functional tests that exercise venv through the angreal CLI.

    These tests create temporary angreal projects with venv-dependent tasks
    and verify the complete workflow works end-to-end.
    """

    @pytest.fixture
    def angreal_project(self, tmp_path):
        """Create a temporary angreal project structure."""
        # Create .angreal directory
        angreal_dir = tmp_path / ".angreal"
        angreal_dir.mkdir()

        # Create minimal angreal.toml
        angreal_toml = tmp_path / "angreal.toml"
        angreal_toml.write_text('project_name = "venv_test_project"\n')

        # Create __init__.py
        init_file = angreal_dir / "__init__.py"
        init_file.write_text("")

        return tmp_path

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows functional tests need different handling",
    )
    def test_venv_required_decorator_functional(self, angreal_project):
        """
        Functional test: Task with @venv_required decorator creates venv.

        This test verifies the complete workflow:
        1. Create an angreal task with @venv_required decorator
        2. Run the task via CLI
        3. Verify venv was created
        4. Verify package was installed and importable
        5. Verify task output is correct
        """
        import subprocess

        # Create a task file that uses @venv_required
        task_file = angreal_project / ".angreal" / "task_venv_functional.py"
        task_content = '''
import angreal
from angreal.integrations.venv import venv_required

@angreal.command(name="venv-test", about="Test venv_required decorator")
@venv_required(".functional_test_venv", requirements="six")
def venv_test_task():
    """Task that runs inside a virtual environment."""
    import six
    # Write proof that we ran in the venv with the package available
    from pathlib import Path
    proof_file = Path(".venv_test_proof.txt")
    proof_file.write_text(f"six_version={six.__version__}")
    print(f"SUCCESS: six version {six.__version__}")
'''
        task_file.write_text(task_content)

        # Run the angreal task
        result = subprocess.run(
            ["angreal", "venv-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120  # Allow time for venv creation and package install
        )

        # Verify task succeeded
        assert result.returncode == 0, f"Task failed with stderr: {result.stderr}"
        assert "SUCCESS" in result.stdout or "six version" in result.stdout

        # Verify venv was created
        venv_path = angreal_project / ".functional_test_venv"
        assert venv_path.exists(), "Virtual environment directory should be created"
        assert (venv_path / "pyvenv.cfg").exists(), "pyvenv.cfg should exist"

        # Verify proof file was written (proves task executed in venv context)
        proof_file = angreal_project / ".venv_test_proof.txt"
        assert proof_file.exists(), "Proof file should be written by task"
        proof_content = proof_file.read_text()
        assert "six_version=" in proof_content, "Proof should contain six version"

        # Cleanup
        shutil.rmtree(venv_path)

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows functional tests need different handling",
    )
    def test_venv_required_with_requirements_file_functional(self, angreal_project):
        """
        Functional test: @venv_required with a requirements.txt file.
        """
        import subprocess

        # Create a requirements.txt file
        req_file = angreal_project / "test_requirements.txt"
        req_file.write_text("toml\n")

        # Create a task file that uses @venv_required with requirements file
        task_file = angreal_project / ".angreal" / "task_venv_reqfile.py"
        task_content = '''
import angreal
from angreal.integrations.venv import venv_required

@angreal.command(name="venv-reqfile-test", about="Test with requirements file")
@venv_required(".reqfile_test_venv", requirements="test_requirements.txt")
def venv_reqfile_task():
    """Task that installs from requirements file."""
    import toml
    from pathlib import Path
    proof_file = Path(".reqfile_test_proof.txt")
    proof_file.write_text(f"toml_version={toml.__version__}")
    print(f"SUCCESS: toml version {toml.__version__}")
'''
        task_file.write_text(task_content)

        # Run the angreal task
        result = subprocess.run(
            ["angreal", "venv-reqfile-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120
        )

        # Verify task succeeded
        assert result.returncode == 0, f"Task failed with stderr: {result.stderr}"

        # Verify venv was created
        venv_path = angreal_project / ".reqfile_test_venv"
        assert venv_path.exists(), "Virtual environment should be created"

        # Verify proof file
        proof_file = angreal_project / ".reqfile_test_proof.txt"
        assert proof_file.exists(), "Proof file should be written"
        assert "toml_version=" in proof_file.read_text()

        # Cleanup
        shutil.rmtree(venv_path)

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows functional tests need different handling",
    )
    def test_venv_required_with_multiple_packages_functional(self, angreal_project):
        """
        Functional test: @venv_required with a list of packages.
        """
        import subprocess

        # Create a task file that uses @venv_required with multiple packages
        task_file = angreal_project / ".angreal" / "task_venv_multi.py"
        task_content = '''
import angreal
from angreal.integrations.venv import venv_required

@angreal.command(name="venv-multi-test", about="Test with multiple packages")
@venv_required(".multi_pkg_venv", requirements=["six", "toml"])
def venv_multi_task():
    """Task that installs multiple packages."""
    import six
    import toml
    from pathlib import Path
    proof_file = Path(".multi_pkg_proof.txt")
    proof_file.write_text(f"six={six.__version__},toml={toml.__version__}")
    print(f"SUCCESS: six={six.__version__}, toml={toml.__version__}")
'''
        task_file.write_text(task_content)

        # Run the angreal task
        result = subprocess.run(
            ["angreal", "venv-multi-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120
        )

        # Verify task succeeded
        assert result.returncode == 0, f"Task failed with stderr: {result.stderr}"

        # Verify venv was created
        venv_path = angreal_project / ".multi_pkg_venv"
        assert venv_path.exists(), "Virtual environment should be created"

        # Verify proof file has both packages
        proof_file = angreal_project / ".multi_pkg_proof.txt"
        assert proof_file.exists(), "Proof file should be written"
        content = proof_file.read_text()
        assert "six=" in content and "toml=" in content

        # Cleanup
        shutil.rmtree(venv_path)

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows functional tests need different handling",
    )
    def test_virtualenv_context_manager_in_task_functional(self, angreal_project):
        """
        Functional test: Using VirtualEnv as context manager within a task.
        """
        import subprocess

        # Create a task file that uses VirtualEnv directly as context manager
        task_file = angreal_project / ".angreal" / "task_venv_context.py"
        task_content = '''
import angreal
from angreal.integrations.venv import VirtualEnv
import subprocess
import sys

@angreal.command(name="venv-context-test", about="Test VirtualEnv context manager")
def venv_context_task():
    """Task that uses VirtualEnv as context manager."""
    from pathlib import Path

    with VirtualEnv(".context_test_venv", requirements="six", now=True) as venv:
        # Run a subprocess that uses the venv's Python
        result = subprocess.run(
            ["python", "-c", "import six; print(six.__version__)"],
            capture_output=True,
            text=True
        )
        six_version = result.stdout.strip()

        # Write proof
        proof_file = Path(".context_test_proof.txt")
        proof_file.write_text(f"six_version={six_version}")
        print(f"SUCCESS: six version from subprocess: {six_version}")
'''
        task_file.write_text(task_content)

        # Run the angreal task
        result = subprocess.run(
            ["angreal", "venv-context-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120
        )

        # Verify task succeeded
        assert result.returncode == 0, f"Task failed with stderr: {result.stderr}"
        assert "SUCCESS" in result.stdout

        # Verify venv was created
        venv_path = angreal_project / ".context_test_venv"
        assert venv_path.exists(), "Virtual environment should be created"

        # Verify proof file
        proof_file = angreal_project / ".context_test_proof.txt"
        assert proof_file.exists(), "Proof file should be written"
        assert "six_version=" in proof_file.read_text()

        # Cleanup
        shutil.rmtree(venv_path)

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows functional tests need different handling",
    )
    def test_venv_reuse_across_invocations_functional(self, angreal_project):
        """
        Functional test: Verify venv is reused (not recreated) on subsequent runs.
        """
        import subprocess

        # Create a task file
        task_file = angreal_project / ".angreal" / "task_venv_reuse.py"
        task_content = '''
import angreal
from angreal.integrations.venv import venv_required
import time

@angreal.command(name="venv-reuse-test", about="Test venv reuse")
@venv_required(".reuse_test_venv", requirements="six")
def venv_reuse_task():
    """Task for testing venv reuse."""
    import six
    print(f"SUCCESS: six version {six.__version__}")
'''
        task_file.write_text(task_content)

        venv_path = angreal_project / ".reuse_test_venv"

        # First run - creates venv
        result1 = subprocess.run(
            ["angreal", "venv-reuse-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120
        )
        assert result1.returncode == 0, f"First run failed: {result1.stderr}"
        assert venv_path.exists()

        # Record creation time of pyvenv.cfg
        pyvenv_cfg = venv_path / "pyvenv.cfg"
        first_mtime = pyvenv_cfg.stat().st_mtime

        # Small delay to ensure mtime would change if recreated
        import time
        time.sleep(0.1)

        # Second run - should reuse existing venv
        result2 = subprocess.run(
            ["angreal", "venv-reuse-test"],
            cwd=angreal_project,
            capture_output=True,
            text=True,
            timeout=120
        )
        assert result2.returncode == 0, f"Second run failed: {result2.stderr}"

        # Verify venv was NOT recreated (mtime unchanged)
        second_mtime = pyvenv_cfg.stat().st_mtime
        assert first_mtime == second_mtime, "Venv should be reused, not recreated"

        # Cleanup
        shutil.rmtree(venv_path)
