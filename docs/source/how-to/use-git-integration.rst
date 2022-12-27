Using Git Integrations
======================

Angreal provides a simple interface for interacting with the ``git`` version control system.

The base git object points to the git binary, with each subcommand being exposed as a method on the object. Simply pass
parameters and options through the method call as a series of arguments for execution.

Full documentation is available |api_git_integrations|. 


.. code-block:: python

    from angreal import Git

    git = Git()
    git.init()
    git.add('this_file.txt')
    git.commit('-m','This file was committed')
    git.push('origin','master')



.. |api_git_integrations| raw:: html

	<a href ="../angreal/angreal.integrations.git.html" target="_blank"> here </a>