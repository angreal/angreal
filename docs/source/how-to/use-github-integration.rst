###################
GitHub Integrations
###################


The Basics
""""""""""

Access to GitHub's API is available throught the |GitHub| class. An access token is required to interact with 
GitHub's service. 

.. note::

    For information on how to get and setup an user authentication token check out GitHub's documentation
    `here <https://help.github.com/en/articles/creating-a-personal-access-token-for-the-command-line>`_. For the sake of this documentation it is 
    assumed that the token is stored in an environmental variable called `GITHUB_TOKEN`.


.. code-block:: python

    from angreal import GitHub
    github = GitHub('http://github.com',token=os.environ.get('GITHUB_TOKEN'))


After initializing the GitHub object, the repository to interact with must be either fetched or created. Using the |get_repo| method 
does require having the integer id from the remote. If you are creating a template that will use this, it is suggested that you store the id inside
of the |Replay| for later retrieval. 

.. code-block:: python

    github.get_repo(1)
    github.create_repository('new_repo')


Once the objects repository has been set, you can begin interacting with it provided methods in the class. :

- |protect_branch|
- |add_label|
- |add_milestone|

.. note:: NoOp methods
	
	While available the following do nothing as GitHub's API does not support the functionality. They will silently pass if called.
	- |enable_issues|
	- |enable_gitlfs|
	- |enable_pipelines|
	- |enable_registry|

.. warning::

    |destroy_project| , is absolutely destructive and does not provide any protections, use at your own risk.


More Advanced Usage
"""""""""""""""""""

Once the GitHub object is initialized the :py:attr:`remote <angreal.integrations.gh.GitHub.remote>` attribute is available for direct manipulation. This is simply a binding to 
a `PyGithub <https://pygithub.readthedocs.io/en/latest/>`_ object, and as such can be used as though interacting with that library directly.



.. |Replay| raw:: html

	<a href="../angreal/angreal.replay.html" target="_blank"> <code class=" xref py py-class docutils literal notranslate"> Replay </code> </a>


.. |GitHub|  raw:: html 

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub" target="_blank"> <code class=" xref py py-class docutils literal notranslate">GitHub</code> </a>


.. |get_repo|  raw:: html 

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.get_repo" target="_blank"> <code class=" xref py py-class docutils literal notranslate">get_repo()</code> </a>


.. |protect_branch|  raw:: html 

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.protect_branch" target="_blank"> <code class=" xref py py-class docutils literal notranslate">protect_branch()</code> </a>


.. |add_label|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.add_label" target="_blank"> <code class=" xref py py-class docutils literal notranslate">add_label()</code> </a>

.. |add_milestone|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.add_milestone" target="_blank"> <code class=" xref py py-class docutils literal notranslate">add_milestone()</code> </a>

.. |enable_issues|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.enable_issues" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_issues()</code> </a>

.. |enable_gitlfs|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.enable_gitlfs" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_gitlfs()</code> </a>

.. |enable_pipelines|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.enable_pipelines" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_pipelines()</code> </a>

.. |enable_registry|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gh.GitHub.enable_registry" target="_blank"> <code class=" xref py py-class docutils literal notranslate">enable_registry()</code> </a>


.. |destroy_project|	raw:: html

	<a href="../angreal/angreal.integrations.gh.html#angreal.integrations.gl.GitHub.destroy_project" target="_blank"> <code class=" xref py py-class docutils literal notranslate">destroy_project()</code> </a>
