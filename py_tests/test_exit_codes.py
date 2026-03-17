"""Tests for task exit code propagation."""
import os
import subprocess

here = os.path.dirname(__file__)
functional_test_folder = os.path.join(here, "functional_tests")


def _run(*args):
    return subprocess.run(
        ["angreal", *args],
        cwd=functional_test_folder,
        capture_output=True,
    )


def test_return_integer_zero():
    """Integer return of 0 should exit 0."""
    rv = _run("exit-zero")
    assert rv.returncode == 0


def test_return_integer_nonzero():
    """Integer return of 42 should exit 42."""
    rv = _run("exit-nonzero")
    assert rv.returncode == 42


def test_return_true():
    """Boolean True should exit 0."""
    rv = _run("exit-true")
    assert rv.returncode == 0


def test_return_false():
    """Boolean False should exit 1."""
    rv = _run("exit-false")
    assert rv.returncode == 1


def test_return_none():
    """None return should exit 0."""
    rv = _run("exit-none")
    assert rv.returncode == 0


def test_sys_exit_zero():
    """sys.exit(0) should exit 0."""
    rv = _run("exit-sys-exit", "--code", "0")
    assert rv.returncode == 0


def test_sys_exit_nonzero():
    """sys.exit(3) should exit 3."""
    rv = _run("exit-sys-exit", "--code", "3")
    assert rv.returncode == 3


def test_exception_exits_56():
    """Unhandled exception should exit 56."""
    rv = _run("exit-exception")
    assert rv.returncode == 56
