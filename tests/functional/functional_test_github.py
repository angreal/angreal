import unittest
import os
import random
import string
import datetime
import time
from angreal.integrations.gh import GitHub





def generate_random_string():
    letters = string.ascii_letters
    return ''.join(random.choice(string.ascii_letters) for i in range(50))


def get_GH_token():
    """
    Attempt to get the GitHub testing key from the testing environment.
    Falls back to a hardcoded key that will need to be manually updated if we lose it. not a huge deal but just one more
    thing to fix


    :return:
    """

    key = os.environ.get('GITHUB_TESTING_KEY')
    if not key:
        key = '385528338a0de4c56fd1e2ac2916b28169d9a8a9'
    return key

class TestGithub(unittest.TestCase):

    def setUp(cls) -> None:
        cls.repo_name = generate_random_string()
        cls.gh = GitHub(access_token=get_GH_token())
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
        github test github create repo
        """
        self.gh.create_repository(self.repo_name)


    def test_get_repo(self):
        """
        github test get_repo
        """

        self.gh.create_repository(self.repo_name)
        created_id = self.gh.repo.id
        self.gh = None
        self.gh = GitHub(access_token=get_GH_token())
        self.gh.get_repo(created_id)
        assert self.gh.repo.name == self.repo_name


    def test_add_label(self):
        """
        github test add_label
        """
        self.gh.create_repository(self.repo_name)
        self.gh.add_label('test','000000')

    def test_add_milestone(self):
        """
        github test add_milestone
        """
        self.gh.create_repository(self.repo_name)
        self.gh.add_milestone(title='Test',description='Test',due_date=datetime.date.today())


    def test_protect_branch(self):
        """
        github test protect branch
        """
        self.gh.create_repository(self.repo_name)
        self.gh.repo.create_file(self.repo_name,self.repo_name,self.repo_name,branch='master')
        self.gh.protect_branch('master')