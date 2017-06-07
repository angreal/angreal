from unittest import TestCase
from angreal import Git
from angreal import GitException
from shutil import rmtree
import tempfile
import os

tempdir = tempfile.gettempdir()
tempgit = os.path.join(tempdir,'.git')

class TestGit(TestCase):
    
    def test_git_1(self):
        """
        test behavior when git_path is None
        """
        try:
            git = Git(git_path=None)
        except OSError:
            pass


    def test_git_1(self):
        """
        test behaviour when git_path doesn't exist
        """
        try:
            git = Git(git_path='not git')
        except OSError:
            pass

    def test_git_3(self):
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
        os.chdir(tempdir)
        assert os.getcwd() == tempdir
        git.init()
        assert os.path.exists('.git')
        rmtree('.git')
    
    def test_git_init_call(self):
        """
        basic test to ensure init works as an argument
        """
        git = Git(working_dir=tempdir)
        os.chdir(tempdir)
        assert os.getcwd() == tempdir
        git('init')
        assert os.path.exists('.git')
        rmtree('.git')

