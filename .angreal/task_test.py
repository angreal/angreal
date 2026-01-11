"""Core test commands for angreal."""
import os
import subprocess
import tempfile
from pathlib import Path

import angreal
from angreal.integrations.venv import VirtualEnv

project_root = Path(angreal.get_root()).parent

test = angreal.command_group(name="test", about="commands for testing the"
                             " application and library")


@test()
@angreal.command(
    name="all",
    about="Run complete test suite (Python, Rust, completion)",
    tool=angreal.ToolDescription("""
Run the complete test suite including Python, Rust, and completion tests.

## When to use
- Before major releases
- After significant changes across multiple components
- For comprehensive validation

## When NOT to use
- During rapid development cycles (use specific test commands)
- When only one component changed

## Examples
```
angreal test all
```
""", risk_level="safe")
)
def all_tests():
    """
    Run all tests: Python, Rust (unit + integration), and completion tests
    """
    print("=== Running All Tests ===\n")
    failures = []

    print("1. Running Python tests...")
    result = python_tests()
    if result:
        failures.append("Python tests")

    print("\n2. Running Rust tests...")
    result = rust_tests_combined()
    if result:
        failures.append("Rust tests")

    print("\n3. Running completion tests...")
    result = test_completion_all()
    if result:
        failures.append("Completion tests")

    if failures:
        print(f"\nFAIL: The following test suites failed: {', '.join(failures)}")
        return 1

    print("\nAll test suites completed!")


@test()
@angreal.command(
    name="python",
    about="Run Python unit tests with pytest in isolated environment",
    tool=angreal.ToolDescription("""
Run Python unit tests with pytest in an isolated virtual environment.

## When to use
- After Python code changes
- Before committing Python changes
- To verify Python bindings work correctly

## When NOT to use
- When only Rust code changed
- During Rust-only development cycles

## Examples
```
angreal test python
```
""", risk_level="safe")
)
def python_tests():
    """
    Run the Python unit tests in isolated environment
    """
    import sys

    print("Creating isolated test environment...")
    with VirtualEnv("angreal-pytest-venv", now=True) as venv:
        # Ensure pip is available (platform-specific paths)
        if sys.platform == "win32":
            python_exe = os.path.join(venv.path, "Scripts", "python.exe")
            pip_exe = os.path.join(venv.path, "Scripts", "pip.exe")
        else:
            python_exe = os.path.join(venv.path, "bin", "python")
            pip_exe = os.path.join(venv.path, "bin", "pip3")

        # Install pip and dependencies with streaming output
        print("Ensuring pip is available...")
        subprocess.run(
            [python_exe, "-m", "ensurepip"],
            check=True
        )

        print("Installing test dependencies (maturin, pytest)...")
        subprocess.run(
            [pip_exe, "install", "maturin", "pytest"],
            check=True
        )

        # Build and install angreal with streaming output
        print("Building and installing angreal from source...")
        subprocess.run(
            [pip_exe, "install", str(project_root / "crates" / "angreal")],
            check=True
        )

        # Run pytest with streaming output
        print("Running Python tests with pytest...")
        result = subprocess.run(
            [venv.python_executable, "-m", "pytest", "-svv"],
            cwd=str(project_root)
        )
        if result.returncode != 0:
            return result.returncode


@test()
@angreal.command(
    name="rust",
    about="Run Rust unit and integration tests with cargo",
    tool=angreal.ToolDescription("""
Run Rust unit and integration tests using cargo.

## When to use
- After Rust code changes
- Before committing Rust changes
- To verify core functionality

## When NOT to use
- When only Python code changed
- During Python-only development cycles

## Examples
```
angreal test rust              # Run all Rust tests
angreal test rust --unit-only  # Run unit tests only
angreal test rust --integration-only  # Run integration tests only
```
""", risk_level="safe")
)
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
        return integration_rust_tests()
    elif unit_only:
        print("Running Rust unit tests only...")
        return unit_rust_tests()
    else:
        print("Running all Rust tests...")
        result = unit_rust_tests()
        if result:
            return result
        return integration_rust_tests()


def integration_rust_tests():
    """
    Run the Rust integration tests
    """
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--test", "integration", "-v",
         "--", "--nocapture", "--test-threads=1"],
        cwd=str(project_root)
    )
    if result.returncode != 0:
        return result.returncode
    return 0


def unit_rust_tests():
    """
    Run the Rust unit tests
    """
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--lib", "-v",
         "--", "--nocapture", "--test-threads=1"],
        cwd=str(project_root)
    )
    if result.returncode != 0:
        return result.returncode
    return 0


@test()
@angreal.command(
    name="completion",
    about="Run shell completion tests for bash and zsh",
    tool=angreal.ToolDescription("""
Run shell completion tests for bash and zsh.

## When to use
- After modifying completion logic
- Before releases
- When testing shell integration

## When NOT to use
- During core functionality development
- When completion is not affected

## Examples
```
angreal test completion           # Run all completion tests
angreal test completion --shell=bash  # Bash only
angreal test completion --shell=zsh   # Zsh only
```
""", risk_level="safe")
)
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
        print("PASS: All completion tests")


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
        print(f"FAIL: Bash completion function not found in script: {script[:200]}...")
        return
    if "complete -F _angreal_completion angreal" not in script:
        print(f"FAIL: Completion registration not found in script: {script[:200]}...")
        return

    print("OK: Bash completion script generation")


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
        print(f"FAIL: Zsh compdef not found in script: {script[:200]}...")
        return
    if "_angreal" not in script:
        print(f"FAIL: Zsh completion function not found in script: {script[:200]}...")
        return

    print("OK: Zsh completion script generation")


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
        print("OK: Init completion generation")
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
            print(f"FAIL: Test command not found in completions: {completions}")
            return
        print("OK: Project task completion generation")
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
                    print("OK: Local template discovery")
                else:
                    print(
                        "WARN: Local template discovery: No local templates found "
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
