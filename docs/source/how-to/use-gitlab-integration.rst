Using Gitlab Integrations
==========================

Gitlab integrations are available. The general work flow is to instantiate a connection to the gitlab host using
a user token from the provider for authentication.

.. note::

    For information on how to get and setup an user authentication token check out Gitlab's documentation
    `here <https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html>`_.



The creation of the Gitlab object includes the authentication against the service provider. As a general security
practice, your token shouldn't be hardcoded into any template and you should expect users to provide their git lab token
through some mechanism.


.. code-block:: python

    from angreal.integrations.gitlab import GitLabProject
    gitlab = GitLabProject('http://gitlab.com',token=os.environ.get('GITLAB_TOKEN'))


From here there are a few routes forward :

* find the name space id of a group

.. code-block:: python

    #Search for a group with an exact match
    group_id = gitlab.get_namespace_id('group55',interactive=False)

    #search for a group with a fuzzy (interactive match)
    group_id = gitlab.get_namespace_id('group55',interactive=False)


* create a new project

.. code-block:: python

    #create a project in your default namespace
    gitlab.create('super_awesome_project')

    #create a project in a specific namespace
    gitlab.create('super_awesome_project', name_space_id=15)


* get an existing project

.. code-block:: python

    #get a project by id number
    gitlab.get_project(15)

    #get a project by specific id, keep in mind the namespace needs to be explicit
    gitlab.get_project('namespace/project_name')

Once you have a project a number of methods/attributes become available to you :

**methods**:

* :meth:`add_milestone() <angreal.integrations.gitlab.GitLabProject.add_milestone>`
* :meth:`add_label() <angreal.integrations.gitlab.GitLabProject.add_label>`
* :meth:`protect_branch() <angreal.integrations.gitlab.GitLabProject.protect_branch>`

**attributes**:

.. warning::

    Class attributes are used to enable common services in Gitlab, once turned on they can't be turned off via this route.

* :meth:`enable_pipelines <angreal.integrations.gitlab.GitLabProject.enable_pipelines>`
* :meth:`enable_gitlfs <angreal.integrations.gitlab.GitLabProject.enable_gitlfs>`
* :meth:`enable_registry <angreal.integrations.gitlab.GitLabProject.enable_registry>`
* :meth:`enable_issues <angreal.integrations.gitlab.GitLabProject.enable_issues>`



