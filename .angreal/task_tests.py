import angreal
import os
import subprocess

venv_path = os.path.join(angreal.get_root(),'..','.venv')

cwd = os.path.join(angreal.get_root(),'..')

test = angreal.command_group(name="test", about="commands for testing the"
                             " application and library")


@test()
@angreal.command(name="rust", about="run cargo tests (rust)")
def rust_tests():
    """
    """
    subprocess.run(
        [
            "cargo test -v -- --nocapture --test-threads=1",
        ], cwd=cwd, shell=True
    )

@test()
@angreal.command(name="python", about="run pytest tests (python)")
def python_tests():
    """
    """
    subprocess.run(
        ["pip install ."], cwd=cwd, shell=True
    )
    subprocess.run(["python3 -m pytest -svv"], cwd=cwd, shell=True)
