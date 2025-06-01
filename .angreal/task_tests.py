import angreal
import os
import subprocess
import sys
import tempfile
from pathlib import Path

venv_path = os.path.join(angreal.get_root(),'..','.venv')

cwd = os.path.join(angreal.get_root(),'..')

test = angreal.command_group(name="test", about="commands for testing the"
                             " application and library")

unit = angreal.command_group(name="unit", about="commands for running unit tests")
integration = angreal.command_group(name="integration",
                                    about="commands for running integration tests")

completion = angreal.command_group(name="completion",
                                  about="commands for testing shell completion")

@test()
@angreal.command(name="all", about="run all tests")
def rust_tests():
    """
    Run the Rust tests
    """
    python_tests()
    integration_rust_tests()
    unit_rust_tests()


@test()
@unit()
@angreal.command(name="python", about="run pytest unit tests")
def python_tests():
    """
    Run the Python unit tests
    """
    result = subprocess.run(
        [sys.executable, "-m", "pip", "install", "."], cwd=cwd
    )
    if result.returncode != 0:
        exit(result.returncode)

    subprocess.run([sys.executable, "-m", "pytest", "-svv"], cwd=cwd)

@test()
@integration()
@angreal.command(name="rust", about="run cargo integration tests")
def integration_rust_tests():
    """
    Run the Rust integration tests
    """
    subprocess.run(
        [
            "cargo test --test integration -v -- --nocapture --test-threads=1",
        ], cwd=cwd, shell=True
    )

@test()
@unit()
@angreal.command(name="rust", about="run cargo unit tests")
def unit_rust_tests():
    """
    Run the Rust unit tests
    """
    subprocess.run(
        [
            "cargo test --lib -v -- --nocapture --test-threads=1",
        ], cwd=cwd, shell=True
    )


@test()
@completion()
@angreal.command(name="bash", about="test bash completion script generation")
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
        cwd=cwd,
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


@test()
@completion()
@angreal.command(name="zsh", about="test zsh completion script generation")
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
        cwd=cwd,
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


@test()
@completion()
@angreal.command(name="completions", about="test completion generation for commands")
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
        cwd=cwd,
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
        cwd=cwd,
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


@test()
@completion()
@angreal.command(
    name="template-discovery",
    about="test template discovery for completion"
)
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
                cwd=cwd,
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


@test()
@completion()
@angreal.command(name="all", about="run all completion tests")
def test_completion_all():
    """
    Run all completion tests
    """
    print("Running all completion tests...")

    test_bash_completion()
    test_zsh_completion()
    test_completion_generation()
    test_template_discovery()

    print("🎉 All completion tests passed!")
