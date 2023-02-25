import os
from angreal.integrations.venv import venv_required, VirtualEnv
import shutil
import sys
import pytest

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_venv_required():
    """
    test venv required good
    """

    @venv_required("__angreal", requirements='flask')
    def test(a, b):
        import flask
        return a + b

    try:
        assert test(1, 2) == 3
    except:
        raise
    finally:
        shutil.rmtree("__angreal")


def test_init():
    """
    testing creation of an environment
    """
    test_requirements = os.path.join(os.path.dirname(__file__), "test_r.txt")

    assert os.path.exists(test_requirements)
    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix

    this_venv = "__test_venv_1"
    assert not os.path.isdir(this_venv)

    venv = VirtualEnv(path=this_venv, requirements=test_requirements, now=True).install_requirements()

    

    try:
        import flask
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except:
            pass
            


def test_requirements_load():
    """
    testing load from "string"
    """

    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix

    this_venv = "__test_venv_2"
    assert not os.path.isdir(this_venv)

    venv = VirtualEnv(path=this_venv, requirements=flask, now=True).install_requirements()

    try:
        import flask
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except:
            pass


def test_requirements_load():
    """
    test load requirements from list
    """
    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix
    this_venv = "__test_venv_3"
    assert not os.path.isdir(this_venv)

    venv = VirtualEnv(path=this_venv, requirements=["flask"], now=True).install_requirements()

    try:
        import flask
    except (ImportError, AssertionError):
        raise
    finally:
        try:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix
        except:
            pass
