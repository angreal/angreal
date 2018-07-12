"""
    angreal.githost
    ~~~~~~~~~~~~~~~

    An abstract base class for describing interactions with a git hosting service (github,gitlab,bitbucket,etc.)
"""


class GitHost(object):

    def __init__(self):
        """
        initialize a connection to the git host

        Should complete through to authorization of the project
        """
        raise NotImplementedError()

    def get_group_id(self):
        """
        get access information for a group level namespace
        :return:
        """
        raise NotImplementedError()

    def create_project(self):
        raise NotImplementedError()

    def protect_branch(self):
        raise NotImplementedError()

    def create_label(self):
        raise NotImplementedError()

    def create_milestone(self):
        raise NotImplementedError()

    def create_issue(self):
        raise NotImplementedError()