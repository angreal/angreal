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

    subprocess.run(["maturin", "develop", "-q"])

    print(green + "====================" + end )
    print(green + "Starting Cargo tests" + end )
    print(green + "====================" + end )

    cargo_rv = subprocess.run(["cargo", "test", "--", "--nocapture", "--test-threads=1", "-v"])

    print(green + "=====================" + end )
    print(green + "Starting python tests" + end)
    print(green + "=====================" + end )
    pytest_rv = subprocess.run(["python","-m","pytest", "-vv"])

    exit(cargo_rv.returncode + pytest_rv.returncode)