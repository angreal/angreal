Your First Angreal
==================



Angreal's are more useful when you build your own or tweak others.

.. note:: This project is available as part of the git repository `here <https://gitlab.com/dylanbstorey/angreal/tree/master/example>`_.

It's a very simple project for taking meeting minutes !

We have some basic requirements for this template :

- on creation a folder with the meeting name will be created
- on creation a file that introduces us to the reason the meetings exist
- a task to take minutes using an editor




Create our "Meeting Name" directory 
""""""""""""""""""""""""""""""""""""

.. code-block:: bash

    $: mkdir example
    $: cd example
    $: mkdir {{angreal.name}}
    $: echo '{ "name" : meeting-minutes }' >> angreal.json

When `example` is rendered, it will create a folder with the name "name". The `angreal.json` file provides a list of the 
variables used during rendering (and default values).


Create our Init Script
""""""""""""""""""""""


.. warning::
    The ``init`` task must:
        - be within the ``.angreal`` folder
        - be named ``init.py``
        - define an function called ``init`` decorated by ``angreal.command()``


When the template is rendered up we'd like a file called ``Introduction.md`` to be created. For this example we'll do this
by invoking some python after rendering our template via the an init script.

First create the file:

.. code-block:: bash

    $: mkdir .angreal
    $: touch .angreal/init.py

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
                print( input("Describe the objective(s) of this meeting series:\n"), file=f)

        return

Let's briefly walk through this code to see whats going on:

    - import angreal and decorating our ``init`` with ``@angreal.command()``
    - add a runtime option ``--no_objectives`` that can be used from the command line
    - we create a file `Introduction.md` and add some text to it.




Create our Task to Take Minutes
"""""""""""""""""""""""""""""""


.. warning::
    Angreal tasks must :
        - be inside of the ``.angreal`` folder
        - have the prefix ``task_``
        - define a function called ``angreal_cmd``


Next we'll create the task that describes how we take minutes

.. code-block:: bash

    $: touch .angreal/task_take_minutes.py

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

Again a brief walk through of the code :
    - import angreal and other libraries we'll be using
    - we decorate a function called ``angreal_cmd`` and add an option ``--now``
    - the ``angreal_cmd``

        + determines the current date and time 
        + tries to get an ``EDITOR`` variable from the environment
        + opens a temporary file using your editor 
        + on exit, saves the notes taken to a file with the date and time the minutes were started.



Using our Angreal
"""""""""""""""""


Let's take our angreal for a spin how this works now.

**Initializing a new set of minutes**

.. code-block:: bash

    $: angreal init example --help

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

    $: angreal init example
    name [meeting-minutes]: hall-of-the-tower
    $: ls
    hall-of-the-tower/
    $ ls hall-of-the-tower/
    Introduction.md

We've created our new project and it was set up with the appropriate ``Introduction.md`` file.

Let's start using our project.

.. code-block:: bash

    $: cd hall-of-the-tower

What commands do i have access to ?

.. code-block:: bash

    $: angreal --help
    Usage: angreal [OPTIONS] COMMAND [ARGS]...

    Options:
      --help  Show this message and exit.

    Commands:
      take_minutes  create a file for taking minutes

How does take_minutes work ?

.. code-block:: bash

    $: angreal take_minutes --help

    Usage: angreal take_minutes [OPTIONS]

      create a file for taking minutes

    Options:
      --now   start taking minutes immediately (requires EDITOR to be set)
      --help  Show this message and exit.


Let's take some minutes, right now

.. code-block:: bash

    $: export EDITOR='vim'
    $: angreal take_minutes --now


This will open a vim editor, write a note to yourself (i) and exit (-Esc-:w:q).


.. code-block:: bash

    $: ls
    2018-06-16-13-12.md  Introduction.md

    $ cat 2018-06-13-12.md
    # 2018-06-16-13-12

    Guys I'd like to spend at least half a book talking about embroidery on dresses, any advice ?





