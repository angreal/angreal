import os
import unittest
from angreal.integrations.virtual_env import venv_required,VirtualEnv
import shutil
import sys
from nose.tools import raises



class TestVirtualEnv(unittest.TestCase):

    def test_venv_required(self):
        """
        test venv required good
        """

        @venv_required('angreal')
        def test(a, b):
            return a + b

        assert test(1, 2) == 3

    @raises(ValueError)
    def test_venv_required_bad(self):
        @venv_required('not_angreal')
        def test(a, b):
            return a + b

        test(1, 2)


    def test_init(self):
        """
        testing creation of an environment
        :return:
        """

        #activation edits sys.prefix, save and reset it when this test passes
        initial_sys_prefix = sys.prefix

        this_venv = os.path.expanduser(os.path.join('~','.venv','test'))
        assert not os.path.isdir(this_venv)

        venv = VirtualEnv(name='test')
        assert venv.base_path == os.path.expanduser(os.path.join('~','.venv'))
        assert venv.path == os.path.join(venv.base_path,venv.name)

        shutil.rmtree(this_venv)
        sys.prefix = initial_sys_prefix
