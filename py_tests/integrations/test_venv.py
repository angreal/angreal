import os
from angreal.integrations.venv import venv_required, VirtualEnv
import shutil
import sys
import pytest


# @pytest.mark.skipif(
#     sys.platform == 'win32'
# )
def test_venv_required():
    """
    test venv required good
    """

    venv = VirtualEnv(path="__angreal", now=True)

    @venv_required("__angreal")
    def test(a, b):
        return a + b

    try:
        assert test(1, 2) == 3
    except:
        raise
    finally:
        shutil.rmtree("__angreal")


def test_venv_required_bad():
    """
    test venv_required missing venv created if not present
    """
    with pytest.raises(EnvironmentError):

        @venv_required("__not_angreal")
        def test(a, b):
            return a + b

    shutil.rmtree("__not_angreal")


def test_init():
    """
    testing creation of an environment
    """
    test_requirements = os.path.join(os.path.dirname(__file__), "test_r.txt")

    assert os.path.exists(test_requirements)
    # activation edits sys.prefix, save and reset it when this test passes
    initial_sys_prefix = sys.prefix

    this_venv = "__test_venv"
    assert not os.path.isdir(this_venv)

    venv = VirtualEnv(path=this_venv, requirements=test_requirements)

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
