Quick Start
===========


Requirements
------------

Angreal has only been tested under python 3.5+, there are no current plans to make it backward compatible.


Installation
------------

.. code-block:: bash

    $: pip install angreal


Usage
-----

.. code-block:: bash

    $: angreal

    Usage: angreal [OPTIONS] COMMAND [ARGS]...

    Options:
      --help  Show this message and exit.

    Currently available commands:
    list                    get a list of currently available commands
    init                    Initialize an angreal based project.





**Initialize :**

.. code-block:: bash

    $: angreal init https://gitlab.com/angreal/meeting-minutes.git

    name [meeting-minutes]: weekly-meeting
    Describe the objective(s) of this meeting series:
    To talk about a plan to have a meeting

    $: ls

    weekly-meeting

**Explore:**

.. code-block:: bash

    $: cd weekly-meeting && angreal

    Usage: angreal [OPTIONS] COMMAND [ARGS]...

    Options:
      --help  Show this message and exit.

    Project Commands:
      take_minutes  create a file for taking minutes

    Globally available commands:
      list                    get a list of currently available commands
      init                    Initialize an angreal based project.


    $: angreal take_minutes --help

    Usage: angreal take_minutes [OPTIONS]

    create a file for taking minutes

    Options:
      --now   start taking minutes immediately (requires EDITOR to be set)
      --help  Show this message and exit.

**Use:**

.. code-block:: bash

    $: export EDITOR='vim'
    $: angreal take_minutes --now


    $: ls -l
    2018-08-09-16-34.md
    Introduction.md

