###################
GitLab Integrations
###################


The Basics
""""""""""

Access to GitLab's API is available throught the |GitLab| class. An access token is required to interact with 
GitLab's service. 

.. note::

    For information on how to get and setup an user authentication token check out Gitlab's documentation
    `here <https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html>`_. For the sake of this documentation it is 
    assumed that the token is stored in an environmental variable called `GITLAB_TOKEN`.


.. code-block:: python

    from angreal import GitLab
    gitlab = GitLab('http://gitlab.com',token=os.environ.get('GITLAB_TOKEN'))


After initializing the GitLab object, the repository to interact with must be either fetched or created. Using the |get_repo| method 
does require having the integer id from the remote. If you are creating a template that will use this, it is suggested that you store the id inside
of the |Replay| for later retrieval. 

.. code-block:: python

    gitlab.get_repo(1)
    gitlab.create_repository('new_repo')


Once the objects repository has been set, you can begin interacting with it provided methods in the class. :

- |protect_branch|
- |add_label|
- |add_milestone|
- |enable_issues|
- |enable_gitlfs|
- |enable_pipelines|
- |enable_registry|

.. warning::

    |destroy_project| , is absolutely destructive and does not provide any protections, use at your own risk.


More Advanced Usage
"""""""""""""""""""

Once the GitLab object is initialized the :py:attr:`remote <angreal.integrations.gl.GitLab.remote>` attribute is available for direct manipulation. This is simply a binding to 
a `python-gitlab <https://python-gitlab.readthedocs.io/en/latest/api-usage.html>`_ object, and as such can be used as though interacting with that library directly.



.. |Replay| raw:: html

	<a href="../angreal/angreal.replay.html" target="_blank"> <code class=" xref py py-class docutils literal notranslate"> Replay </code> </a>


.. |GitLab|  raw:: html 

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab" target="_blank"> <code class=" xref py py-class docutils literal notranslate">GitLab</code> </a>


.. |get_repo|  raw:: html 

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.get_repo" target="_blank"> <code class=" xref py py-class docutils literal notranslate">get_repo()</code> </a>


.. |protect_branch|  raw:: html 

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.protect_branch" target="_blank"> <code class=" xref py py-class docutils literal notranslate">protect_branch()</code> </a>


.. |add_label|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.add_label" target="_blank"> <code class=" xref py py-class docutils literal notranslate">add_label()</code> </a>

.. |add_milestone|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.add_milestone" target="_blank"> <code class=" xref py py-class docutils literal notranslate">add_milestone()</code> </a>

.. |enable_issues|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.enable_issues" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_issues()</code> </a>

.. |enable_gitlfs|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.enable_gitlfs" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_gitlfs()</code> </a>

.. |enable_pipelines|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.enable_pipelines" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_pipelines()</code> </a>

.. |enable_registry|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.enable_registry" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_registry()</code> </a>


.. |destroy_project|	raw:: html

	<a href="../angreal/angreal.integrations.gl.html#angreal.integrations.gl.GitLab.destroy_project" target="_blank"> <code class=" xref py py-class docutils literal notranslate">destroy_project()</code> </a>
