import angreal
import os
import subprocess
import sys

venv_path = os.path.join(angreal.get_root(),'..','.venv')

cwd = os.path.join(angreal.get_root(),'..')

test = angreal.command_group(name="test", about="commands for testing the"
                             " application and library")

unit = angreal.command_group(name="unit", about="commands for running unit tests")
integration = angreal.command_group(name="integration",
                                    about="commands for running integration tests")

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
