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
