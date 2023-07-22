import angreal
from angreal.integrations.venv import venv_required
import os
import subprocess


cwd = os.path.join(angreal.get_root(),'..')

green = "\33[32m"
red = "\33[31m"
end = "\33[0m"

venv_path = os.path.join(angreal.get_root(),'..','.venv')

@angreal.command(name="run-tests", about="run our test suite")
@venv_required(path=venv_path, requirements=['maturin','pre-commit','pytest'])
def run_tests():
    """
    Run tests for both cargo test and pytest
    """
    print(green + "====================" + end)
    print(green + "building" + end)
    print(green + "====================" + end)

    subprocess.run(
        ["maturin build"], cwd=cwd, shell=True
    )

    subprocess.run(
        ["pip install ."], cwd=cwd, shell=True
    )

    print(green + "====================" + end)
    print(green + "Starting Cargo tests" + end)
    print(green + "====================" + end)

    cargo_rv = subprocess.run(
        [
            "cargo test -v -- --nocapture --test-threads=1",
        ], cwd=cwd, shell=True
    )

    print(green + "=====================" + end)
    print(green + "Starting python tests" + end)
    print(green + "=====================" + end)
    pytest_rv = subprocess.run(["python3 -m pytest -svv"], cwd=cwd, shell=True)

    if cargo_rv.returncode or pytest_rv.returncode:
        raise RuntimeError(
            f"Tests failed with status codes : {cargo_rv}"
            " (cargo) and {pytest_rv}(pytest)"
        )
