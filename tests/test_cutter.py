import filecmp
import os
import shutil
import unittest

from tests import return_to_cwd
from angreal.cutter import initialize_cutter


class TestCutter(unittest.TestCase):

    @return_to_cwd
    def test_good_rep(self):
        """
        Just making sure that our cookie-cutter pass through works as intended
        :return:
        """


        os.chdir(os.path.dirname(__file__))

        initialize_cutter('fake-repo-pre',no_input=True)
        assert filecmp.cmp(os.path.join('fake-project', 'README.rst'),
                           os.path.join('fake-repo', 'fake-project', 'README.rst'))

        assert os.path.isfile(os.path.join('fake-project', '.angreal', 'angreal-replay.json'))

        shutil.rmtree('fake-project')

