Interact With ``virtualenv``
============================

Integration with ``virtualenv`` centers around two activities :

* ensuring that a specific environment is activated before running

.. code-block:: python

    from angreal.integrations.virtual_env import venv_required

    @venv_required('virtual_environment')
    def only_run_if_active():
        print('Hello World')



* set up a virtual environment

.. code-block:: python

    from angreal.integrations.virtual_env import VirtualEnv

    #create a venv using python3
    venv = VirtualEnv(python='python3')


