import filecmp
import os
import shutil
import unittest


from angreal.cutter import initialize_cutter


class TestCutter(unittest.TestCase):

    def test_good_rep(self):
        """
        Just making sure that our cookie-cutter pass through works as intended
        :return:
        """

        original_dir = os.getcwd()
        os.chdir(os.path.dirname(__file__))

        initialize_cutter('fake-repo-pre',no_input=True)
        assert filecmp.cmp(os.path.join('fake-project', 'README.rst'),
                           os.path.join('fake-repo', 'fake-project', 'README.rst'))

        assert os.path.isfile(os.path.join('fake-project', '.angreal', 'angreal-replay.json'))

        shutil.rmtree('fake-project')
        os.chdir(original_dir)
