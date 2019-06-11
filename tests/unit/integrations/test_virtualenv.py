import os
import unittest
from angreal.integrations.virtual_env import venv_required,VirtualEnv
import shutil
import sys
from nose.tools import raises

test_requirements = os.path.join(os.path.dirname(__file__),'test_r.txt')

class TestVirtualEnv(unittest.TestCase):

    def test_venv_required(self):
        """
        test venv required good
        """

        @venv_required('angreal')
        def test(a, b):
            return a + b

        assert test(1, 2) == 3

    @raises(EnvironmentError)
    def test_venv_required_bad(self):
        """
        teste venv_required missing venv created if not present
        """

        @venv_required('not_angreal')
        def test(a, b):
            return a + b



    def test_init(self):
        """
        testing creation of an environment
        """

        #activation edits sys.prefix, save and reset it when this test passes
        initial_sys_prefix = sys.prefix

        this_venv = os.path.expanduser(os.path.join('~','.venv','test'))
        assert not os.path.isdir(this_venv)

        venv = VirtualEnv(name='test', requirements=test_requirements)

        try:
            import flask
            assert venv.base_path == os.path.expanduser(os.path.join('~', '.venv'))
            assert venv.path == os.path.join(venv.base_path, venv.name)
        except (ImportError,AssertionError):
            raise
        finally:
            shutil.rmtree(this_venv)
            sys.prefix = initial_sys_prefix




