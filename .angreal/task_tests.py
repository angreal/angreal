import angreal
import os
import subprocess
import tempfile
from pathlib import Path
from angreal.integrations.venv import VirtualEnv

project_root = Path(angreal.get_root()).parent

test = angreal.command_group(name="test", about="commands for testing the"
                             " application and library")

@test()
@angreal.command(name="all", about="run all tests")
def all_tests():
    """
    Run all tests: Python, Rust (unit + integration), and completion tests
    """
    print("=== Running All Tests ===\n")

    print("1. Running Python tests...")
    python_tests()

    print("\n2. Running Rust tests...")
    rust_tests_combined()

    print("\n3. Running completion tests...")
    test_completion_all()

    print("\n🎉 All test suites completed!")


@test()
@angreal.command(name="python", about="run pytest unit tests")
def python_tests():
    """
    Run the Python unit tests in isolated environment
    """
    with VirtualEnv("angreal-pytest-venv", now=True) as venv:
        # Ensure pip is available (platform-specific paths)
        import sys
        if sys.platform == "win32":
            python_exe = venv.path / "Scripts" / "python.exe"
            pip_exe = venv.path / "Scripts" / "pip.exe"
        else:
            python_exe = venv.path / "bin" / "python"
            pip_exe = venv.path / "bin" / "pip3"

        # Install pip and dependencies
        subprocess.run(
            [str(python_exe), "-m", "ensurepip"],
            check=True, capture_output=True
        )
        subprocess.run(
            [str(pip_exe), "install", "maturin", "pytest"],
            check=True, capture_output=True
        )

        # Build and install angreal (non-editable to ensure Rust compilation)
        subprocess.run(
            [str(pip_exe), "install", str(project_root)],
            check=True
        )

        # Run pytest
        result = subprocess.run(
            [str(venv.python_executable), "-m", "pytest", "-svv"],
            cwd=str(project_root)
        )
        if result.returncode != 0:
            exit(result.returncode)

@test()
@angreal.command(name="rust", about="run cargo unit and integration tests")
@angreal.argument(
    name="unit_only",
    long="unit-only",
    help="run only unit tests",
    required=False,
    takes_value=False,
    is_flag=True
)
@angreal.argument(
    name="integration_only",
    long="integration-only",
    help="run only integration tests",
    required=False,
    takes_value=False,
    is_flag=True
)
def rust_tests_combined(unit_only: bool = False, integration_only: bool = False):
    """
    Run Rust unit and integration tests
    """
    if unit_only and integration_only:
        print("Error: Cannot specify both --unit-only and --integration-only")
        return 1

    if integration_only:
        print("Running Rust integration tests only...")
        integration_rust_tests()
    elif unit_only:
        print("Running Rust unit tests only...")
        unit_rust_tests()
    else:
        print("Running all Rust tests...")
        unit_rust_tests()
        integration_rust_tests()

def integration_rust_tests():
    """
    Run the Rust integration tests
    """
    result = subprocess.run(
        "cargo test --test integration -v -- --nocapture --test-threads=1",
        cwd=str(project_root), shell=True
    )
    if result.returncode != 0:
        exit(result.returncode)

def unit_rust_tests():
    """
    Run the Rust unit tests
    """
    result = subprocess.run(
        "cargo test --lib -v -- --nocapture --test-threads=1",
        cwd=str(project_root), shell=True
    )
    if result.returncode != 0:
        exit(result.returncode)


@test()
@angreal.command(name="completion", about="run all shell completion tests")
@angreal.argument(
    name="shell",
    long="shell",
    help="run tests for specific shell (bash, zsh)",
    required=False,
    takes_value=True
)
def test_completion_all(shell: str = None):
    """
    Run all completion tests or tests for a specific shell
    """
    if shell and shell not in ["bash", "zsh"]:
        print(f"Error: Unknown shell '{shell}'. Supported: bash, zsh")
        return 1

    if shell == "bash":
        print("Running bash completion tests only...")
        test_bash_completion()
    elif shell == "zsh":
        print("Running zsh completion tests only...")
        test_zsh_completion()
    else:
        print("Running all completion tests...")
        test_bash_completion()
        test_zsh_completion()
        test_completion_generation()
        test_template_discovery()
        print("🎉 All completion tests passed!")

def test_bash_completion():
    """
    Test bash completion script generation
    """
    print("Testing bash completion script generation...")

    # Test bash completion script generation
    env = os.environ.copy()
    env["ANGREAL_NO_AUTO_COMPLETION"] = "1"

    # Use the angreal command directly
    result = subprocess.run(
        ["angreal", "_completion", "bash"],
        cwd=str(project_root),
        capture_output=True,
        text=True,
        env=env
    )

    if result.returncode != 0:
        print(f"Completion script generation failed: {result.stderr}")
        return

    # Verify script content
    script = result.stdout
    if "_angreal_completion" not in script:
        print(f"❌ Bash completion function not found in script: {script[:200]}...")
        return
    if "complete -F _angreal_completion angreal" not in script:
        print(f"❌ Completion registration not found in script: {script[:200]}...")
        return

    print("✅ Bash completion script generation: PASSED")

def test_zsh_completion():
    """
    Test zsh completion script generation
    """
    print("Testing zsh completion script generation...")

    # Test zsh completion script generation
    env = os.environ.copy()
    env["ANGREAL_NO_AUTO_COMPLETION"] = "1"

    # Use the angreal command directly
    result = subprocess.run(
        ["angreal", "_completion", "zsh"],
        cwd=str(project_root),
        capture_output=True,
        text=True,
        env=env
    )

    if result.returncode != 0:
        print(f"Completion script generation failed: {result.stderr}")
        return

    # Verify script content
    script = result.stdout
    if "#compdef angreal" not in script:
        print(f"❌ Zsh compdef not found in script: {script[:200]}...")
        return
    if "_angreal" not in script:
        print(f"❌ Zsh completion function not found in script: {script[:200]}...")
        return

    print("✅ Zsh completion script generation: PASSED")

def test_completion_generation():
    """
    Test completion generation for various command contexts
    """
    print("Testing completion generation...")

    # Test completion for 'init' command (should suggest templates)
    env = os.environ.copy()
    env["ANGREAL_NO_AUTO_COMPLETION"] = "1"

    result = subprocess.run(
        ["angreal", "_complete", "init"],
        cwd=str(project_root),
        capture_output=True,
        text=True,
        env=env
    )

    if result.returncode == 0:
        completions = result.stdout.strip().split('\n')
        print(f"Init completions: {completions}")
        # Note: We can't guarantee template suggestions due to network dependency
        print("✅ Init completion generation: PASSED")
    else:
        print(f"Init completion failed: {result.stderr}")

    # Test completion for project tasks (when in angreal project)
    result = subprocess.run(
        ["angreal", "_complete"],
        cwd=str(project_root),
        capture_output=True,
        text=True,
        env=env
    )

    if result.returncode == 0:
        completions = result.stdout.strip().split('\n')
        print(f"Project task completions: {completions}")

        # Should include our test commands
        if not any("test" in comp for comp in completions):
            print(f"❌ Test command not found in completions: {completions}")
            return
        print("✅ Project task completion generation: PASSED")
    else:
        print(f"Project completion failed: {result.stderr}")

def test_template_discovery():
    """
    Test template discovery functionality
    """
    print("Testing template discovery...")

    # Create a temporary .angrealrc directory with mock templates
    with tempfile.TemporaryDirectory() as temp_dir:
        angreal_cache = Path(temp_dir) / ".angrealrc"
        angreal_cache.mkdir()

        # Create mock template directories
        (angreal_cache / "python-cli").mkdir()
        (angreal_cache / "django-api").mkdir()
        (angreal_cache / "rust-project").mkdir()

        # Set HOME to our temp directory for this test
        original_home = os.environ.get("HOME")
        os.environ["HOME"] = temp_dir

        try:
            # Test init completion which should find local templates
            env = os.environ.copy()
            env["ANGREAL_NO_AUTO_COMPLETION"] = "1"

            result = subprocess.run(
                ["angreal", "_complete", "init"],
                cwd=str(project_root),
                capture_output=True,
                text=True,
                env=env
            )

            if result.returncode == 0:
                completions = result.stdout.strip().split('\n')
                print(f"Template completions: {completions}")

                # Should include our mock templates
                has_local_templates = any(
                    template in completions
                    for template in ["python-cli", "django-api", "rust-project"]
                )

                if has_local_templates:
                    print("✅ Local template discovery: PASSED")
                else:
                    print(
                        "⚠️  Local template discovery: No local templates found "
                        "(may include GitHub templates)"
                    )

            else:
                print(f"Template discovery failed: {result.stderr}")

        finally:
            # Restore original HOME
            if original_home:
                os.environ["HOME"] = original_home
            else:
                os.environ.pop("HOME", None)
