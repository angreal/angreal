==================
An Example Project
==================

The easiest way to demo this tool is to just build an angreal, and then use it.


Our Project
===========
This project is available as part of the git repository `here <https://gitlab.com/dylanbstorey/angreal/tree/master/example>`_.
It's a very simple project for taking meeting minutes !



Create our Project Template
---------------------------
As mentioned previously the project template is a cookie cutter.

- ``mkdir example``
- ``cd example``
- ``mkdir {{cookiecutter.name}}``
- ``echo '{ "name" : meeting-minutes }' >> cookiecutter.json``

Our basic template is complete, when created we'll have a folder that's simply the name of the meeting series we attend.


Create our Init Script
----------------------

.. note::
    This could get handled with cookie cutter prehooks or as an actual jinja temlpate in cookiecutter. This isn't a particularly
    good example of an intro script but should give you the basic ideas to make things that are infinitely more useful.

When the project is set up we'd like a file called ``Introduction.md`` to be created.


- ``mkdir .angreal``
- ``touch .angreal/init.py``

Open ``.angreal/init.py`` in your favorite editor and add the following code.

.. code-block:: python
    :linenos:

    import angreal

    @angreal.command()
    @angreal.option('--no_objectives',is_flag=True, help="These meetings are pointless")
    def init(no_objectives):
        """
        Initialize your meetings project.
        """

        with open('Introduction.md','w') as f:
            print('Meeting Objectives', file=f)
            if not no_objectives:
                print( input("Describe the objective(s) of this meeting series"), file=f)

        return

Let's briefly walk through this code to see whats going on:

    - we import angreal
    - we define signal that the following callable(s) are angreal commands
    - we create an option for this command
    - we define a command called ``init``

.. warning::
    The ``init`` task must:
        - be within the ``.angreal`` folder
        - be named ``init.py``
        - import ``angreal``
        - define an ``angreal.command`` called ``init``



Create our Task to Take Minutes
-------------------------------
    - touch ``.angreal/task_take_minutes.py``

Open your favorite editor and add the following code to ``.angreal/task_take_minutes.py``.


.. code-block:: python
    :linenos:

    import angreal

    import datetime
    import os
    import subprocess
    import tempfile


    @angreal.command()
    @angreal.option('--now',is_flag=True,help='start taking minutes immediately.')
    def angreal_cmd(now):
        """
        create a file for taking minutes
        """
        file_name = datetime.datetime.today().strftime('%Y-%m-%d-%H-%M')

        editor = os.environ.get('EDITOR',None)

        # Create our default file template using the current time as a header
        (fd, path) = tempfile.mkstemp()
        with open(fd, 'w') as default:
            print('# {}'.format(file_name), file=default)

        # We want to start writing now if we're able
        if now and editor:
            subprocess.call('{} {}'.format(editor,path), shell=True)


        # Send the finalized contents of the temporary file to the actual file
        with open(file_name+'.md', 'a') as dst:
            with open(path,'r') as src:
                print(src.read(),file=dst)

        # Clean up behind our selves
        os.unlink(path)


.. warning::
    Angreal tasks must :
        - be inside of the ``.angreal`` folder
        - have the prefix ``task_``
        - define a function called ``angreal_cmd``


Using our Angreal
#################

Let's take see how this works now.

**Initializing a new set of minutes**

.. code-block:: bash

    $: angreal init angreal/example --help

    Usage:  [OPTIONS] REPOSITORY [INIT_ARGS]...

      Initialize an angreal based project.

    Options:
      -h, --help  Display a help message

        These are the options for the repository (angreal/example) you are attempting to initialize

    Usage:  [OPTIONS]

      Initialize your meetings project.

    Options:
      --no_objectives  These meetings are pointless
      --help           Show this message and exit.


This angreal template creates a meetings project and the initialization command has the option ``--no_objectives``. I'm
going to assume that we're not holding meetings for the sake of it so lets create a new meeting series.


.. code-block:: bash

    $: angreal init angreal/example
    name [meeting-minutes]: hall-of-the-tower
    $: ls
    hall-of-the-tower/
    $ ls hall-of-the-tower/
    Introduction.md

We've created our new project and it was set up with the appropriate ``Introduction.md`` file.

Let's start using our project.

.. code-block:: bash

    $: cd hall-of-the-tower

    # What commands do i have access to ?

    $: angreal --help
    Usage: angreal [OPTIONS] COMMAND [ARGS]...

    Options:
      --help  Show this message and exit.

    Commands:
      take_minutes  create a file for taking minutes

    # How does take_minutes work ?
    $: angreal take_minutes --help

    Usage: angreal take_minutes [OPTIONS]

      create a file for taking minutes

    Options:
      --now   start taking minutes immediately (requires EDITOR to be set)
      --help  Show this message and exit.


    # Let's take some minutes, right now

    $: export EDITOR='vim'
    $: angreal take_minutes --now

    # This will open a vim editor, write a note to yourself (i) and exit (-Esc-:w:q).

    $: ls
    2018-06-16-13-12.md  Introduction.md

    $ cat 2018-06-13-12.md
    # 2018-06-16-13-12

    Guys I'd like to spend at least half a book talking about embroidery on dresses, any advice ?





