import angreal
import subprocess


@angreal.command(name="run-tests", about="Run the angreal testing harness")
def run_tests():
    subprocess.run(
        [ 'cargo', 'test', '-q', '--', '--test-threads=1' ]
    )
    subprocess.run(
        ['pytest', '-v']
    )
    return

@angreal.command(name="fail")
def fail():
    print('wee')
    raise TypeError("this")