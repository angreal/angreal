import os
import unittest
from nose.tools import raises

from angreal.utils import get_angreal_path
from angreal import warn,win,error

class TestUtils(unittest.TestCase):

    def test_get_angreal_task_path(self):
        """
        test that we can find the path to the projects angreal directory
        :return:
        """
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        path = get_angreal_path(dir='.angreal')
        assert os.path.exists(path)
        os.chdir(original_dir)


    def test_from_not_root(self):
        """
        test that we recurse up correctly
        """
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo','fake-project'))
        path = get_angreal_path(dir='.angreal')
        assert os.path.exists(path)
        os.chdir(original_dir)


    @raises(FileNotFoundError)
    def test_get_angreal_task_path_bad(self):
        """
        bad file path raises FileNotFoundError
        """

        get_angreal_path(dir='.noangreal')

    def test_warn(self):
        """
        Test a warning
        """

        warn("THIS IS A WARNING")

    def test_error(self):
        """
        Test an error
        """

        error("THIS IS AN ERROR")

    def test_win(self):
        """
        Test a win
        """

        win("WINNING")