import unittest
import os
import subprocess
import sys
import shutil
HERE = os.path.dirname(__file__)

setup_py = os.path.abspath(os.path.join(HERE,'..','..'))


class TestAngrealInit(unittest.TestCase):

    @classmethod
    def setUpClass(cls) -> None:
        rc = subprocess.call([sys.executable, '-m', 'pip', 'install', setup_py])
        assert rc==0

    def test_init(self):
        """
        test basic init
        """
        rc = subprocess.call(['angreal', 'init', '--no-input', os.path.join(HERE,'..','unit','fake-repo-pre')])
        assert os.path.isdir('fake-project')
        assert rc == 0
        shutil.rmtree('fake-project')


    def test_init_pypi(self):
        """
        test pypi init - $HOME currently breaks this in GitLab Runner. Come back to it later
        """
        rc = subprocess.call(['angreal','init','--no-input', 'template'])
        # assert os.path.isdir('angreal-template')
        # assert rc == 0
        # shutil.rmtree('angreal-template')






