"""Tasks for testing exit code propagation."""
import sys
import angreal


@angreal.command(name="exit-zero", about="return integer 0")
def exit_zero():
    return 0


@angreal.command(name="exit-nonzero", about="return integer 42")
def exit_nonzero():
    return 42


@angreal.command(name="exit-true", about="return True")
def exit_true():
    return True


@angreal.command(name="exit-false", about="return False")
def exit_false():
    return False


@angreal.command(name="exit-none", about="return None")
def exit_none():
    return None


@angreal.command(name="exit-sys-exit", about="call sys.exit with code")
@angreal.argument(name="code", long="code", short="c", python_type="int", required=True)
def exit_sys_exit(code):
    sys.exit(code)


@angreal.command(name="exit-exception", about="raise a generic exception")
def exit_exception():
    raise RuntimeError("something went wrong")
