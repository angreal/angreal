"""
    angreal.integrations.gl
    ~~~~~~~~~~~~~~~~~~~~~~~

    working with gitlab hosted remotes
"""

import sys
import os
import datetime

from angreal.integrations import GitRemote,repo_required

from gitlab import Gitlab
from gitlab import DEVELOPER_ACCESS, MASTER_ACCESS,OWNER_ACCESS



class GitLab(GitRemote): # pragma: no cover
    def __init__(self, base_url='https://gitlab.com', access_token=None):

        super(GitLab, self).__init__()

        # Do we have http_proxy variables set ?
        proxy = os.environ.get("https_proxy",
                               os.environ.get("http_proxy", None))


        if proxy : # if we detected proxy vars, respect them
            session = requests.Session()
            session.proxies = {
                "https": os.environ.get("https_proxy",None),
                "http": os.environ.get("http_proxy",None),
            }
            self.remote = Gitlab(url=base_url, private_token=access_token, session=session)
        else :
            self.remote = Gitlab(url=base_url, private_token=access_token)

        self.remote.auth()


    def get_repo(self, id):
        """
        Get a repo via canonical id
        :param id:
        :return:
        """
        self.repo = self.remote.projects.get(int(id))
        pass

    def create_repository(self, name, namespace=None):
        """
        Create a repository

        :param name:
        :param namespace:
        :return:
        """
        assert not self.repo


        if not namespace:
            self.repo = self.remote.projects.create({'name' : name})

        else:
            self.repo = self.remote.projects.create({'name' : name ,
                                                'namespace_id': self.remote.namespaces.get(namespace).id })

    @repo_required
    def protect_branch(self, name, **kwargs):

        merge = kwargs.pop('merge','developer')
        push  = kwargs.pop('push','master')

        access_mapper = {
            'developer': DEVELOPER_ACCESS,
            'master': MASTER_ACCESS,
            'owner': OWNER_ACCESS,
            'none': '0'
        }

        self.repo.protectedbranches.create({
            'name': name,
            'merge_access_level': access_mapper[merge],
            'push_access_level': access_mapper[push]
        })
        pass

    @repo_required
    def add_label(self, name, color, pass_on_fail=True):

        if not color.startswith('#'):
            color = '#' + color

        try:
            self.repo.labels.create({
                "name": name,
                "color": color
            })
        except Exception as e:
            if pass_on_fail:
                print('Unable to add label {} : {}'.format(name,color),file=sys.stderr)
                print(e,file=sys.stderr)
            else :
                raise
        pass

    @repo_required
    def add_milestone(self, title, description=None, due_date=None, start_date=None):
        """
        Add a milestone to the project

        :param title:
        :param description:
        :param due_date:
        :param start_date:
        :return:
        """

        def validate(date_text):
            """
            validate the format of input date string
            :param date_text:
            :type date_text: str
            :return:
            """
            try:
                datetime.datetime.strptime(date_text, '%Y-%m-%d')
            except ValueError:
                raise ValueError("Incorrect data format, should be YYYY-MM-DD")

        if isinstance(start_date,(datetime.datetime, datetime.date)):
            start_date = start_date.strftime('%Y-%m-%d')


        if isinstance(due_date,(datetime.datetime, datetime.date)):
            due_date = due_date.strftime('%Y-%m-%d')

        if start_date:
            validate(start_date)

        if due_date:
            validate(due_date)

        milestone = dict(title=title, description=description, due_date=due_date, start_date=start_date)

        milestone = {k: v for k, v in milestone.items() if v}

        self.repo.milestones.create({
            **milestone
            })

    @repo_required
    def enable_issues(self):
        self.repo.issues_enabled = True
        self.repo.save()

    @repo_required
    def enable_gitlfs(self):
        self.repo.lfs_enabled = True

    @repo_required
    def destroy_project(self):
        self.repo.delete()

    @repo_required
    def enable_pipelines(self):
        self.repo.jobs_enabled = True
        self.repo.save()

    @repo_required
    def enable_registry(self):
        self.repo.container_registry_enabled = True
        self.repo.save()



