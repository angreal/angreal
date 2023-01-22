import angreal

import subprocess


green = "\33[32m"
red = "\33[31m"
end = "\33[0m"
@angreal.command(name='run-tests', about='run our test suite')
def run_tests():
    """
    Run tests for both cargo test and pytest
    """

    print(green + "====================" + end )
    print(green + "Starting Cargo tests" + end )
    print(green + "====================" + end )

    cargo_rv = subprocess.run(["cargo", "test", "-v", "--", "--nocapture", "--test-threads=1", ])

    print(green + "=====================" + end )
    print(green + "Starting python tests" + end)
    print(green + "=====================" + end )
    pytest_rv = subprocess.run(["python","-m","pytest", "-vv"])

    if cargo_rv.returncode or pytest_rv.returncode:
        raise RuntimeError(f"Tests failed with status codes : {cargo_rv} (cargo) and {pytest_rv}(pytest)")

