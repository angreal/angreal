"""
    angreal.integrations.gitlab
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Project creation and management within Gitlab servers

    .. todo::
        * Write proper integration tests for this class
        * Lazy methods for update followed by a single api call

"""
import gitlab
import os
import requests
from functools import wraps

def project_required(f):

    @wraps(f)
    def wrapper(*args,**kwargs):
        if isinstance(args[0], GitLabProject):
            if args[0].project:
                f(*args,**kwargs)
            else :
                raise ValueError('project attribute must be set')
        else:
            raise ValueError('This does not appear to be a GitLabProject')

    return wrapper


class GitLabProject(object): #pragma: no cover
    """
    A class for interacting with projects on GitLab
    """

    def __init__(self,url,token=None,proxy=False):
        """
        initialize a connection to Gitlab

        :param url: the url for the gitlab instance
        :param token: the private token to use for accessing the api
        """

        # Set up a request session to respect proxy variables
        if proxy :
            session = requests.Session()
            session.proxies = {
                "https": os.environ.get("https_proxy",None),
                "http": os.environ.get("http_proxy",None),
            }
            self.gl = gitlab.Gitlab(url=url, private_token=token, session=session)
        else :
            self.gl = gitlab.Gitlab(url=url, private_token=token)

        self.gl.auth()
        self.project = None

        return


    def get_project(self, id):
        """
        Get the project from the server by name or id.

        :param id: the project id or path
        """
        self.project = gl.projects.get(id)
        return self.project

    def create_project(self, name, name_space_id=None):
        """
        Create a project from the remote server.

        :param name: The name of the project
        :param name_space_id: The id of the name space the project should be set within.
        """
        if self.project :
            raise ValueError('Project ID already set, not creating another project within this class instance.')

        if not name_space_id:
            self.project = self.gl.projects.create({'name' : name})

        else:
            self.project = self.gl.projects.create({'name' : name ,
                                                'namespace_id' : name_space_id})

        return self.project

    @project_required
    def add_runner(self,i):
        """
        Add a runner on the project.

        :param i: the id of the runner to add to the project
        :type i: int
        """

        self.project.runners.create({ 'runner_id' : i })



    def protect_branch(self, name, merge='developer', push='master'):
        """
        Protect a branch on the project.
        """
        if self.project:
            access_mapper = {
                'developer' : gitlab.DEVELOPER_ACCESS,
                'master'    : gitlab.MASTER_ACCESS,
                'owner'     : gitlab.OWNER_ACCESS
            }

            self.project.protectedbranches.create({
                'name' : name,
                'merge_access_level' : access_mapper[merge],
                'push_access_level'  : access_mapper[push]
            })


    def add_label(self, name, color):
        """
        Add a label to the project.

        :param name: 
        :param color: 
        """
        if self.project:
            self.project.labels.create({
                "name" : name,
                "color" : color
            })



    def add_milestone(self, title,description=None, due_date=None, start_date=None):
        """
        Create a milestone for the project.

        :param title:
        :param description:
        :param due_date: YYYY-MM-DD
        :param start_date: YYYY-MM-DD
        """

        milestone = dict(title=title, description=description,due_date=due_date,start_date=None)

        milestone = { k : v for k,v in milestone.items() if v}


        if self.project:
            self.project.milestones.create({
                **milestone
            })
            self.project.save()

    # Project Settings
    def enable_pipelines(self):
        """
        Enable ci-cd pipelines
        """
        if self.project:
            self.project.jobs_enabled = True
            self.project.save()

    def enable_gitlfs(self):
        """
        Enable git-lfs
        """
        if self.project:
            self.project.lfs_enabled = True
            self.project.save()

    def enable_registry(self):
        """
        Enable container registry
        """
        if self.project:
            self.project.container_registry_enabled = True
            self.project.save()

    def enable_issues(self):
        """
        Enable issues
        """
        if self.project:
            self.project.issues_enabled = True
            self.project.save()

    def enable_merge_if_pipeline_succeeds(self):
        """
        Enable merge only if pipeline succeeds
        """
        if self.project:
            self.project.only_allow_merge_if_pipeline_succeeds = True
            self.project.save()

    def destroy_project(self):
        """
        destroy the project
        :return:
        """
