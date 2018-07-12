from angreal.integrations.githost import GitHost
import gitlab



class GitLab(GitHost):

    def __init__(self,url,token=None):
        """
        initialize a connection to Gitlab
        :param url:
        :param token:
        """
        self.gl = gitlab.Gitlab(url=url, private_token=token)

        pass

    def get_group_id(self):
        self.available_namespaces = gl.namespaces.list()
        pass

    def create_project(self, repo_name, namespace_id):
        """
        Create a project on a remote
        :param repo_name:
        :param namespace_id:
        :return:
        """
        self.project = gl.projects.create({'name' : repo_name, 'namespace_id': namespace_id})
        pass

    def protect_branch(self, name):
        self.project.protected
        pass

    def create_label(self):
        pass

    def create_milestone(self):
        pass

    def create_issue(self):
        pass




