"""
    angreal.integrations.gitlab
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Project creation and management within Gitlab servers

    .. todo::
        * Lazy methods for update followed by a single api call

"""
import datetime

import gitlab
import os
import sys
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

    Starting with a new project ::

        project = GitLabProject('http://gitlab.com',token='SECRET')
        project.create_project('projectname')

    Starting with a previous project ::

        project = GitLabProject('http://gitlab.com',token='SECRET')
        project.get_project('group/projectname')
        #or project.get_project(1) if you have the actual id

    """

    def __init__(self,url,token=None,proxy=False):
        """
        initialize with a connection to Gitlab

        :param url: the url for the gitlab instance
        :type url: str
        :param token: the private token to use for accessing the api
        :type token: str
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
        :type id:  int
        """
        self.project = gl.projects.get(int(id))
        return self.project

    def create_project(self, name, name_space_id=None):
        """
        Create a project from the remote server.

        :param name: The name of the project
        :type name: str
        :param name_space_id: The id of the name space the project should be set within.
        :type name_space_id: int
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
    def add_runner(self, i, pass_on_fail=True):
        """
        Add a runner on the project.

        :param i: the id of the runner to add to the project
        :type i: int
        """
        try:
            self.project.runners.create({'runner_id': i})
        except gitlab.exceptions.GitlabCreateError as e:
            if pass_on_fail:
                print('Unable to add runner {} to project'.format(i), file=sys.stderr)
                print(e, file=sys.stderr)
            else :
                raise

    @project_required
    def protect_branch(self, name, merge='developer', push='master'):
        """
        protect a branch through gitlab

        :param name: branch regex
        :type name: str
        :param merge: who can merge
        :type merge:
        :param push: who can push
        :return:
        """
        access_mapper = {
            'developer': gitlab.DEVELOPER_ACCESS,
            'master': gitlab.MASTER_ACCESS,
            'owner': gitlab.OWNER_ACCESS,
            'none': '0'
        }

        self.project.protectedbranches.create({
            'name': name,
            'merge_access_level': access_mapper[merge],
            'push_access_level': access_mapper[push]
        })


    @project_required
    def add_label(self, name, color, pass_on_fail=True):
        """
        Add a label to the project.

        :param name: the name of the label
        :param color: the color for the label (must be hex)
        """

        try:
            self.project.labels.create({
                "name": name,
                "color": color
            })
        except (gitlab.exceptions.GitlabCreateError, AssertionError) as e:
            if pass_on_fail:
                print('Unable to add label {} : {}'.format(name,color),file=sys.stderr)
                print(e,file=sys.stderr)
            else :
                raise

    @project_required
    def add_milestone(self, title, description=None, due_date=None, start_date=None):
        """
        Create a milestone for the project.

        :param title:
        :param description:
        :param due_date: YYYY-MM-DD
        :type due_date: str, datetime.datetime, datetime.date
        :param start_date: YYYY-MM-DD
        :type start_date: str, datetime.datetime, datetime.date
        """

        if isinstance(start_date,(datetime.datetime, datetime.date)):
            start_date = start_date.strftime('%Y-%m-%d')
            pass

        if isinstance(due_date,(datetime.datetime, datetime.date)):
            due_date = due_date.strftime('%Y-%m-%d')
            pass

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

        validate(start_date)
        validate(due_date)

        milestone = dict(title=title, description=description, due_date=due_date, start_date=start_date)

        milestone = {k: v for k, v in milestone.items() if v}

        self.project.milestones.create({
            **milestone
        })
        self.project.save()


    def get_namespace_id(self, namespace, interactive=False):
        """
        Get the id of a namespace given a name. Provides functionality to be interactive if multiple namespaces found.


        :return:
        """

        namespace_id = self.gl.namespaces.list(search=namespace)

        # Nothing found
        if not namespace_id:
            raise ValueError('The namespace {} can not be found'.format(namespace))


        # Multiple matches found
        if len(namespace_id) > 1 :
            if interactive: # user picked based on input
                possible_ids = [ (x.name , x.id ) for x in namespace_id]
                possible_ids.append(('None','-1'))
                print('\n'.join(['{}. {}'.format(i,v[0]) for i,v in enumerate(possible_ids)]))


                selection = None
                while not selection:
                    try:
                        selection = int(input('Multiple matching groups found please select from one of the above'))
                    except ValueError:
                        print('Selection must be an integer')
                        selection = None
                        continue
                    try:
                        namespace_id = possible_ids[selection][1] # <- namespace integer set
                    except IndexError:
                        print('Selection out of range')
                        selection = None
                        continue

                if namespace_id == -1 :
                    raise ValueError('You selected None')

            return namespace_id #<-  return list or integer

        else : #perfect match
            return namespace_id[0].id # return the integer


# Properties below the line
    @property
    @project_required
    def enable_pipelines(self):
        """
        Enable ci-cd pipelines
        """
        self.project.jobs_enabled = True
        self.project.save()


    @property
    @project_required
    def enable_gitlfs(self):
        """
        Enable git-lfs
        """

        self.project.lfs_enabled = True
        self.project.save()

    @property
    @project_required
    def enable_registry(self):
        """
        Enable container registry
        """
        self.project.container_registry_enabled = True
        self.project.save()

    @property
    @project_required
    def enable_issues(self):
        """
        Enable issues
        """
        self.project.issues_enabled = True
        self.project.save()

    @property
    @project_required
    def enable_merge_if_pipeline_succeeds(self):
        """
        Enable merge only if pipeline succeeds
        """
        self.project.only_allow_merge_if_pipeline_succeeds = True
        self.project.save()

    @property
    @project_required
    def destroy_project(self):
        """
        destroy the project
        """
        self.project.delete()
        self.project = None