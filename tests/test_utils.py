import os
import unittest
from nose.tools import raises

from angreal.utils import get_angreal_task_path


class TestUtils(unittest.TestCase):

    def test_get_angreal_task_path(self):
        """
        find a file that exists
        :return:
        """
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        file = get_angreal_task_path(file=os.path.join('.angreal', 'angreal_tasks.py'))
        assert os.path.exists(file)
        os.chdir(original_dir)

    @raises(FileNotFoundError)
    def test_get_angreal_task_path_bad(self):
        """
        bad file path raises FileNotFoundError
        :return:
        """

        get_angreal_task_path(file='.angreal/no_angreal_tasks.py')
