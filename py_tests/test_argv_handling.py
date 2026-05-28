"""Regression tests for argv handling in angreal.main().

These tests guard against the musl-libc panic where std::env::args() returned
an empty Vec from a dlopened PyO3 extension and split_off(2) crashed with
"2 > 0". The fix is to source argv from Python's sys.argv (always populated
by the interpreter) instead of std::env::args() (depends on libc behavior
in shared-object init).

We run each scenario in a fresh subprocess because angreal.main() has
process-level side effects (logger init, completion install, command parse)
that don't compose across calls.
"""
import os
import subprocess
import sys
import textwrap


def _run(snippet, cwd=None):
    """Run a Python snippet in a subprocess and return (returncode, stdout, stderr)."""
    proc = subprocess.run(
        [sys.executable, "-c", textwrap.dedent(snippet)],
        cwd=cwd,
        capture_output=True,
        text=True,
        timeout=60,
    )
    return proc.returncode, proc.stdout, proc.stderr


def test_main_with_only_script_in_argv_does_not_panic(tmp_path):
    """sys.argv = ['angreal'] (no user args) must not panic.

    This is the exact shape std::env::args() would yield on musl from a
    dlopened .so when ARGV is unpopulated — historically it triggered the
    split_off(2) panic. With sys.argv as the source, it should produce
    normal help output and a clean exit.
    """
    snippet = """
        import sys
        sys.argv = ['angreal']
        import angreal
        try:
            angreal.main()
        except SystemExit as e:
            sys.exit(e.code if e.code is not None else 0)
    """
    rc, out, err = _run(snippet, cwd=str(tmp_path))
    # Must NOT contain the historical panic signature.
    assert "panicked" not in err.lower(), f"argv handling panicked: {err}"
    assert "split_off" not in err, f"split_off panic regression: {err}"


def test_main_with_subcommand_via_sys_argv(tmp_path):
    """`sys.argv = ['angreal', 'tree']` outside a project: clean exit, not panic."""
    snippet = """
        import sys
        sys.argv = ['angreal', 'tree']
        import angreal
        try:
            angreal.main()
        except SystemExit as e:
            sys.exit(e.code if e.code is not None else 0)
    """
    rc, out, err = _run(snippet, cwd=str(tmp_path))
    assert "panicked" not in err.lower(), f"argv handling panicked: {err}"


def test_cli_invocation_runs_normally(tmp_path):
    """End-to-end: `angreal tree` via the console script must not panic.

    This is the integration-level check that mirrors what real users hit
    on a musl wheel. We don't assert exact output because behavior varies
    by whether the cwd has .angreal/, but we DO assert there's no Rust
    panic.
    """
    # Use the same Python that's running the tests to find its console script.
    bin_dir = os.path.dirname(sys.executable)
    angreal_bin = os.path.join(bin_dir, "angreal")
    if not os.path.exists(angreal_bin):
        # Fall back to `python -m` style — skip if neither available.
        proc = subprocess.run(
            [sys.executable, "-c", "import angreal; angreal.main()"],
            cwd=str(tmp_path),
            capture_output=True,
            text=True,
            timeout=60,
        )
    else:
        proc = subprocess.run(
            [angreal_bin, "tree"],
            cwd=str(tmp_path),
            capture_output=True,
            text=True,
            timeout=60,
        )
    assert "panicked" not in proc.stderr.lower(), (
        f"angreal CLI panicked: {proc.stderr}"
    )
    assert "split_off" not in proc.stderr, (
        f"split_off panic regression: {proc.stderr}"
    )
