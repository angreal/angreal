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
    """test that a task can define --verbose / -v without conflicting with angreal's global verbose"""
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
