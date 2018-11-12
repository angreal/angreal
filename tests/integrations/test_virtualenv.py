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

    def test_venv_required_bad(self):
        initial_sys_prefix = sys.prefix

        @venv_required('not_angreal',requirements_file=test_requirements)
        def test(a, b):
            return a + b

        test(1, 2)
        this_venv = os.path.expanduser(os.path.join('~', '.venv', 'not_angreal'))
        assert os.path.isdir(this_venv)

        shutil.rmtree(this_venv)
        sys.prefix = initial_sys_prefix


    def test_init(self):
        """
        testing creation of an environment
        :return:
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




