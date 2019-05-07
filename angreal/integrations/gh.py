"""

    angreal.integrations.github
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Working with github hosted remotes

"""

from angreal.integrations import GitRemote
import sys

from github import Github


class GitHub(GitRemote): # pragma: no cover
    """

    Basic manipulations to a remote github project.

    """

    def __init__(self,base_url='https://api.github.com',access_token=None):
        super(GitHub, self).__init__()
        self.remote = Github(base_url=base_url,login_or_token=access_token)


    def get_repo(self,id):
        """
        Get our namespace by integer id.

        :param id:
        :return:
        """

        self.repo = self.remote.get_repo(id)


    def create_repository(self,name,namespace=None):
        """
        Create a repository on the remote, sets the `repo` attribute on return.

        :param name:
        :param namespace:
        :return:
        """

        assert not self.repo
        if namespace:
            self.repo = self.remote.get_organization(namespace).create_repo(name)
        else :
            self.repo = self.remote.get_user().create_repo(name)

    def protect_branch(self, name, **kwargs):
        """
        Turn branch protection on for a specific branch

        :param name:
        :param kwargs:
        :return:
        """
        self.repo.get_branch(name).edit_protection(True)
        pass

    def add_label(self, name, color, pass_on_fail=True):
        """
        Add an issue label to the project

        :param name:
        :param color:
        :param pass_on_fail:
        :return:
        """

        if color.startswith('#'):
            color = color[1:]
        try:
            self.repo.create_label(name,color)
        except Exception as e:
            if pass_on_fail:
                print('Unable to add label {} : {}'.format(name, color), file=sys.stderr)
                print(e, file=sys.stderr)
            else:
                raise

    def add_milestone(self, title, description=None, due_date=None, start_date=None):
        """
        Add a milestone to the project

        :param title:
        :param description:
        :param due_date:
        :param start_date:
        :return:
        """
        self.repo.create_milestone(title,description=description,due_on=due_date)
        pass

    def enable_issues(self):
        """
        Issues are enabled by default in GitHub

        :return:
        """
        pass

    def enable_gitlfs(self):
        """
        GitLFS is enabled by default in GitHub. (Bandwidth limits are in place at the account level)

        :return:
        """
        pass

    def destroy_project(self):
        """
        Delete a project, this is destructive so use with caution.
        :return:
        """
        self.repo.delete()




