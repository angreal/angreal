"""
    angreal.integrations
    ~~~~~~~~~~~~~~~~~~~~

"""
from functools import wraps


def repo_required(f):
    @wraps(f)
    def wrapper(*args,**kwargs):
        if isinstance(args[0], GitRemote):
            if args[0].repo:
                f(*args,**kwargs)
            else :
                raise ValueError('project attribute must be set')
        else:
            raise ValueError('This does not appear to be a GitLabProject')
    return wrapper

class GitRemote(object): # pragma: no cover
    """
    Abstract Base Class for working with git remotes repository hosts
    """


    def __init__(self):
        """
        We expect all Remotes to have a base url and an access token.

        :param base_url:
        :param access_token:
        """
        self.__remote = None
        self.__repo = None



    @property
    def remote(self):
        return self.__remote

    @remote.setter
    def remote(self, value):
        if self.__remote:
            raise ValueError("Remote can only be set once !")
        self.__remote = value

    @property
    def repo(self):
        return self.__repo

    @repo.setter
    def repo(self,value):
        if self.__repo:
            raise ValueError("Repo can only be set once !")
        self.__repo = value



    def get_repo(self,id):
        """
        Get a repository by some canonical id

        :param self:
        :return:
        """

        raise NotImplementedError


    def create_repository(self,name,namespace=None):
        """
        Create a repo

        :param name: The name of the project
        :param namespace: A namespace (group/organization) to create the project within. Should default to a default "user" space.
        :return:
        """

        raise NotImplementedError

    def protect_branch(self,name,**kwargs):
        """
        Protect a branch, this usually blocks pushes and deletions.

        :param name: branch name/pattern to protect
        :param **kwargs: provider specific settings
        :return:
        """

        raise NotImplementedError


    def add_label(self,name,color,pass_on_fail=True):
        """
        Attempt to add a label to the project issue tracker

        :param name:
        :param color:
        :param pass_on_fail:
        :return:
        """

        raise NotImplementedError


    def add_milestone(self,title,description=None,due_date=None,start_date=None):
        """
        Add a milestone/deadline to the project.

        :param title:
        :param description:
        :param due_date:
        :param start_date:
        :return:
        """


        raise NotImplementedError


    def enable_issues(self):
        """
        Enable built in issue tracking

        :return:
        """

        raise NotImplementedError


    def enable_gitlfs(self):
        """
        enable git-lfs based file tracking

        :return:
        """

        raise NotImplementedError


    def destroy_project(self):
        """
        destroy a project on the remote server

        :return:
        """

        raise NotImplementedError