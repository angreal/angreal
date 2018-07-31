import os
import sys
import unittest
import mock

import docker
import gitlab
from nose.tools import raises
import polling
import requests
from bs4 import BeautifulSoup

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
            timeout=720, # shouldn't take more than a minute to come up but lets get big
            ignore_exceptions=(requests.exceptions.ConnectionError,)
        )

        print('gitlab up and running - starting tests')

        cls.project = GitLabProject(url=gitlab_hostname,token=generate_token())

        cls.group = cls.project.gl.groups.create({'name': 'group1', 'path': 'group1'})
        cls.project.gl.groups.create({'name' : 'group2', 'path' : 'group2'})



    @classmethod
    def tearDownClass(cls):
        """
        Not completely necessary, but we're good cooks
        """
        if os.environ.get('TEARDOWN_WAIT',None):
            input('Hit enter to teardown')
        cls.container.remove(force=True)


    def function_test_01_no_project(self):
        """
        test no project exists on startup
        :return:
        """
        assert not self.project.project


    def function_test_02_no_project(self):
        """
        decorator works as intended
        """

        try :
            self.project.enable_pipelines
        except ValueError:
            pass

    def function_test_03_create_project(self):
        """
        project creation
        """
        self.project.create_project('test_project', name_space_id=self.group.id)
        assert self.project.project

    def function_test_04_enable_pipeline(self):
        """
        enable pipelines
        """
        self.project.enable_pipelines


    def function_test_05_enable_lfs(self):
        """
        enable git lfs
        """
        self.project.enable_gitlfs

    def function_test_06_enable_registry(self):
        """
        enable docker registry
        """
        self.project.enable_registry

    def function_test_07_enable_issues(self):
        """
        enable registry
        """
        self.project.enable_issues

    def function_test_08_enable_merge_if_succeed(self):
        """
        enable merge only if pipeline succeeds
        """
        self.project.enable_merge_if_pipeline_succeeds

    def function_test_09_test_protect_branch(self):
        """
        protect a branch
        """
        self.project.protect_branch('master',push='none')


    def function_test_10_add_runner(self):
        """
        add runner
        """
        self.project.add_runner(1)

    @raises(gitlab.exceptions.GitlabCreateError)
    def function_test_11_add_runner_fail(self):
        """
        add runner , pass on fail = False
        """
        self.project.add_runner(1, pass_on_fail=False)

    def function_test_12_add_label(self):
        """
        add label
        """
        self.project.add_label(name='test',color='#112233')


    @raises(gitlab.exceptions.GitlabCreateError)
    def function_test_13_add_label_fail(self):
        """
        add label, pass on fail = False

        """
        self.project.add_label(name='test2', color='notacolor', pass_on_fail=False)

    @raises(gitlab.exceptions.GitlabCreateError)
    def function_test_14_add_label_fail(self):
        """
        add label, multiple copies

        """
        self.project.add_label(name='test', color='notacolor',pass_on_fail=False)


    def funtion_test_15_get_groups(self):
        """
        get single group by name
        """
        group_id = self.project.get_namespace_id('group1',interactive=False)
        assert isinstance(group_id,int)

    @raises(ValueError)
    def function_test_16_get_bad_group(self):
        """
        group doesn't exist

        """
        self.project.get_namespace_id('group55',interactive=False)

    def function_test_17_get_fuzzy_group(self):
        """
        multiple groups
        """
        groups = self.project.get_namespace_id('group',interactive=False)
        assert isinstance(groups,list)

    @mock.patch('builtins.input',side_effect=['Nothing',4,1])
    def function_test_18_user_input(self,input):
        """
        interactive inputs
        """
        group = self.project.get_namespace_id('group',interactive=True)
        assert isinstance(group,int)

    def function_test_99_test_delete(self):
        """
        test that we can delete a project
        """
        self.project.destroy_project
        assert not self.project.project


