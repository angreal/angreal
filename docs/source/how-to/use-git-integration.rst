Using Git Integrations
======================

Angreal provides a simple interface for interacting with `git`.

The base git object points to the git binary, with each subcommand being exposed as a method on the object. Simply pass
parameters and options through the method call as a series for execution.



.. code-block:: python

    from angreal.integrations.git import Git

    git = Git()

    git.init()
    git.add('this_file.txt')
    git.commit('-m','This file was committed')
    git.push('origin','master')



