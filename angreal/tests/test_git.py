from unittest import TestCase
from angreal import Git
from angreal import GitException
from shutil import rmtree
import os


tempdir = os.path.abspath( os.path.join( os.path.dirname(__file__) , 'tmp'))
tempgit = os.path.join(tempdir, '.git')


class TestGit(TestCase):

    @classmethod
    def setUpClass(cls):
        os.mkdir(tempdir)
        git = Git()
        assert os.path.exists(tempdir)

    @classmethod
    def tearDownClass(cls):
        rmtree(tempdir)


    def test_git_frombicate(self):
        """
        test git with a nonsense method that git doesn't have
        """

        git = Git()
        try:
            git.frombicate()
        except GitException:
            pass

    def test_git_init(self):
        """
        basic tests to ensure that init works as a method call
        """
        git = Git(working_dir=tempdir)
        git.init()
        assert os.path.exists(tempgit)
        rmtree(tempgit)

    def test_git_init_call(self):
        """
        basic test to ensure init works as an argument
        """
        git = Git(working_dir=tempdir)
        git('init')
        assert os.path.exists(tempgit)
        rmtree(tempgit)


