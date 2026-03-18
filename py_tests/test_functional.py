import os
import subprocess

here = os.path.dirname(__file__)
functional_test_folder = os.path.join(here,"functional_tests")


def test_group_1():
    """test a basic nested command with flag"""
    rv = subprocess.run([
        "angreal",
        "group1",
        "flag",
        "-t"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder,"group.txt"))
    except Exception as e:
        raise e
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder,"group.txt"))
        except Exception:
            pass
    pass


def test_group_2():
    """test a multi nested command with flag"""
    rv = subprocess.run([
        "angreal",
        "group1",
        "group2",
        "flag2",
        "-t"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder,"nested_group.txt"))
    except Exception as e:
        raise e
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder,"nested_group.txt"))
        except Exception:
            pass
    pass


def test_task_verbose_flag():
    """test that a task can define --verbose / -v flags"""
    # Test with --verbose
    rv = subprocess.run([
        "angreal",
        "verbose-test",
        "--verbose"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder, "verbose_test.txt"))
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder, "verbose_test.txt"))
        except Exception:
            pass

    # Test with -v short flag
    rv = subprocess.run([
        "angreal",
        "verbose-test",
        "-v"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder, "verbose_test.txt"))
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder, "verbose_test.txt"))
        except Exception:
            pass


def test_top_level_build():
    """test top-level 'build' runs with its own --release arg"""
    rv = subprocess.run(
        ["angreal", "build", "--release"],
        cwd=functional_test_folder, capture_output=True, text=True,
    )
    marker = os.path.join(
        functional_test_folder, "top_build_release.txt"
    )
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(marker)
    finally:
        try:
            os.unlink(marker)
        except Exception:
            pass


def test_docs_build():
    """test grouped 'docs build' runs with --format (not --release)"""
    rv = subprocess.run(
        ["angreal", "docs", "build", "--format", "pdf"],
        cwd=functional_test_folder, capture_output=True, text=True,
    )
    marker = os.path.join(
        functional_test_folder, "docs_build_pdf.txt"
    )
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(marker)
    finally:
        try:
            os.unlink(marker)
        except Exception:
            pass


def test_docs_build_no_cross_contamination():
    """test 'docs build' doesn't get --release from top-level 'build'"""
    # If args cross-contaminate, this will error with
    # "unexpected argument" or create the wrong marker file.
    rv = subprocess.run([
        "angreal", "docs", "build"
    ], cwd=functional_test_folder, capture_output=True, text=True)
    expected = os.path.join(
        functional_test_folder, "docs_build_html.txt"
    )
    unwanted = os.path.join(functional_test_folder, "top_build.txt")
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(expected), (
            "docs build should create docs_build_html.txt"
        )
        assert not os.path.exists(unwanted), (
            "docs build should NOT trigger top-level build"
        )
    finally:
        for f in [expected, unwanted]:
            try:
                os.unlink(f)
            except Exception:
                pass
