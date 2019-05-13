import unittest
import os
from angreal.task_helpers import *
from nose.tools import raises

class TestTaskHelpers(unittest.TestCase):

    def test_check_environment(self):
        """
        test check environment
        """
        os.environ.setdefault('TEST','test')
        check_environment('TEST')
        check_environment('test')
        os.environ.pop('TEST')
        return

    @raises(EnvironmentError)
    def test_bad_environment(self):
        """
        test check environment bad
        """
        check_environment('test')


    def test_file_exists(self):
        """
        test check files
        """
        check_files('dodo.py')

    @raises(FileNotFoundError)
    def test_file_no_exist(self):
        """
        test check files failes
        """
        check_files('REALLYSHOULDNTEXIST')


    def test_copy(self):
        """
        test copy file
        """
        open('test','w')
        copy_files('test2','test')
        assert os.path.isfile('test')
        assert os.path.isfile('test2')
        os.unlink('test')
        os.unlink('test2')

    def test_clean_files(self):
        """
        test clean files
        """
        open('test','w')
        assert os.path.isfile('test')
        clean_files('test')
        assert not os.path.isfile('test')

    def test_project_source_files(self):
        """
        test project source files
        """
        for f in  get_project_source_files('angreal'):
            assert os.path.isfile(f)
