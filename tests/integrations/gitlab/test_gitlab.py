import sys


from bs4 import BeautifulSoup
import docker
import requests
import polling
import unittest

try:
    from urllib.parse import urljoin
except ImportError:
    from urlparse import urljoin


from angreal.integrations.gitlab import GitLabProject


gitlab_hostname='http://localhost:8080'

endpoint = "http://localhost:8080"
root_route = urljoin(endpoint, "/")
sign_in_route = urljoin(endpoint, "/users/sign_in")
pat_route = urljoin(endpoint, "/profile/personal_access_tokens")

login = "root"
password = "5iveL!fe"


def find_csrf_token(text):
    soup = BeautifulSoup(text, "lxml")
    token = soup.find(attrs={"name": "csrf-token"})
    param = soup.find(attrs={"name": "csrf-param"})
    data = {param.get("content"): token.get("content")}
    return data


def obtain_csrf_token():
    r = requests.get(root_route)
    token = find_csrf_token(r.text)
    return token, r.cookies


def sign_in(csrf, cookies):
    data = {
        "user[login]": login,
        "user[password]": password,
    }
    data.update(csrf)
    r = requests.post(sign_in_route, data=data, cookies=cookies)
    token = find_csrf_token(r.text)
    return token, r.history[0].cookies


def obtain_personal_access_token(name, csrf, cookies):
    data = {
        "personal_access_token[name]": name,
        "personal_access_token[scopes][]": ["api", "sudo"],
    }
    data.update(csrf)
    r = requests.post(pat_route, data=data, cookies=cookies)
    soup = BeautifulSoup(r.text, "lxml")
    token = soup.find('input', id='created-personal-access-token').get('value')
    return token


def generate_token():
    """
    Generate the private token for the api
    :return: private token
    """
    csrf1, cookies1 = obtain_csrf_token()
    csrf2, cookies2 = sign_in(csrf1, cookies1)

    token = obtain_personal_access_token('default', csrf2, cookies2)
    return(token)



class FunctionTestGitLab(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        """
        Before we run our tests, setup our docker container.
        :return:
        """
        cls.client = docker.from_env()
        cls.container = cls.client.containers.run('gpocentek/test-python-gitlab:latest',detach=True, name='gitlab-test', ports = {'22':'2222','80':'8080'})

        print('Waiting for gitlab host to start', file=sys.stderr)

        polling.poll(
            lambda: requests.get('http://localhost:8080/users/sign_in').status_code == 200, #is it up yet ?
            step=5, # check every five seconds
            timeout=360, # shouldn't take more than a minute to come up but lets get big
            ignore_exceptions=(requests.exceptions.ConnectionError,)
        )

        cls.here = GitLabProject(url=gitlab_hostname,token=generate_token())


    @classmethod
    def tearDownClass(cls):
        """
        Not completely necessary, but we're good cooks
        """
        cls.container.remove(force=True)


    def test_01_no_project(self):
        """
        test no project exists on startup
        :return:
        """
        assert not self.here.project