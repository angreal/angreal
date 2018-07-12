import unittest
import os
import  shutil

from angreal.integrations.git import Git, GitException



class TestGit(unittest.TestCase):

    def test_git_1(self):
        """
        test that object fails without path
        """
        try:
            git = Git(git_path=None)
        except OSError:
            pass


    def test_git_1(self):
        """
        test that object fails with bad path
        """
        try:
            git = Git(git_path='not git')
        except OSError:
            pass

    def test_git_3(self):
        """
        test that object fails with bad sub-command
        """
        git = Git()

        try:
            git.frombicate()
        except GitException:
            pass


    def test_git_4(self):
        os.mkdir('git_test')
        os.chdir('git_test')
        git = Git()
        git.init()

        assert os.path.isdir('.git')


        os.chdir('..')
        shutil.rmtree('git_test')


