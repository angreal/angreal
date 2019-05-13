import unittest
import os
import random
import string
import datetime
import time

from angreal.integrations.gl import GitLab





def generate_random_string():
    letters = string.ascii_letters
    return ''.join(random.choice(string.ascii_letters) for i in range(50))


def get_GL_token():
    """
    Attempt to get the GitHub testing key from the testing environment.
    Falls back to a hardcoded key that will need to be manually updated if we lose it. not a huge deal but just one more
    thing to fix


    :return:
    """

    key = os.environ.get('GITLAB_TESTING_KEY')
    if not key:
        key = 'P7bR7NZqgxdxA-iR27H_'
    return key

class TestGithub(unittest.TestCase):

    def setUp(cls) -> None:
        cls.repo_name = generate_random_string()
        cls.gh = GitLab(access_token=get_GL_token())
        pass


    def tearDown(cls) -> None:
        """
        if a repo was created , attempt to clean it up
        """
        if cls.gh.repo:
            time.sleep(2)
            cls.gh.destroy_project()
        pass


    def test_create_repo(self):
        """
        gitlab test create repo
        """
        self.gh.create_repository(self.repo_name)


    def test_get_repo(self):
        """
        gitlab test get_repo
        """

        self.gh.create_repository(self.repo_name)
        created_id = self.gh.repo.id
        self.gh = None
        self.gh = GitLab(access_token=get_GL_token())
        self.gh.get_repo(created_id)
        assert self.gh.repo.name == self.repo_name


    def test_add_label(self):
        """
        gitlab test add_label
        """
        self.gh.create_repository(self.repo_name)
        self.gh.add_label('test','000000')

    def test_add_milestone(self):
        """
        gitlab test add_milestone
        """
        self.gh.create_repository(self.repo_name)
        self.gh.add_milestone(title='Test',description='Test',due_date=datetime.date.today())


    def test_protect_branch(self):
        """
        gitlab test protect branch
        """
        self.gh.create_repository(self.repo_name)
        self.gh.repo.upload(self.repo_name,filedata='HELLO')
        self.gh.protect_branch('master')