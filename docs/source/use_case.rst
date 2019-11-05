################
Use Case
################



Why use this tool?
===================

Imagine you make a practice of writing lots of software packages either as an individual or as part of a team. Every time you have to create the directory and file structure, you probably
do this by hand. You probably do it just a little differently each time, making small inconsistencies on each project. Now imagine you also have to work on multiple software packages and each
one is set up a little differently, with slightly different way of doing basic tasks like running unit tests. You would spend at least some amount of resources : remembering the differences between
each project or fixing little mistakes between them.

The whole point of angreal is to allow you to forget how to set up a project AND how to interact with it. By using templates for both project structure AND administration, you don't have to remember
how to do a task - just that it can be done.



How do I use them ?
===================

You interact with an angreal in two steps :

1. **Initialization** : Initializing an angreal means rendering a described template into a usable form. Templates can exist in one of three locations : local directories, remote git repositories, or pypi modules. Where an angreal template is distributed from is decided by whomever made that particular template.

 .. code-block:: bash

    # initialize and render the template angreal in pypi
    $: angreal init python3

    # initialize and render the template angreal at the remote URI
    $: angreal init https://gitlab.com/angreal/python3.git

    # clone a local copy of the template angreal from github and render that local copy
    $: git clone https://gitlab.com/angreal/python3.git && git init python3

2. **Execution** : Executing an angreal task through the ``angreal`` command line interface. This can only happen after the template has been rendered.

.. code-block:: bash

    # From within an angreal based template
    $: angreal <command to run>



A Quick Example
================

Let's say you're starting a new software project and plan on using `python 3`. You also want to make sure that you're
following good practices and have documentation, testing, and type hinting as part of the project.

You identify that the python3 angreal fits your needs well and choose to use it.


1. You initialize the project with :

.. code-block:: bash

    $: angreal init python3

2. You provide values for template variables with the interactive CLI :

.. code-block:: bash

    source
    name [package-name]: next_big_thing
    author [joesmith]: Jane Smith
    author_email [joesmit@email.com]: JaneSmith@email.com

3. Angreal creates a project for you, using your provided variables to fill in some of the blanks.

.. code-block:: bash

    .
    ├── LICENSE
    ├── MANIFEST.in
    ├── README.rst
    ├── conftest.py
    ├── docs
    │   ├── Makefile
    │   └── source
    │       ├── _static
    │       │   └── logo.png
    │       ├── conf.py
    │       └── index.rst
    ├── next_big_thing
    │   ├── VERSION
    │   ├── __init__.py
    │   └── cli.py
    ├── requirements
    │   ├── dev.txt
    │   └── requirements.txt
    ├── setup.py
    └── tests
        ├── fixtures.py
        ├── integration
        │   └── __init__.py
        └── unit
            └── __init__.py

    8 directories, 17 files

4. Next lets find out what tasks come with the template

.. code-block:: bash

	$: cd next_big_thing
	$: angreal list

	Usage: angreal [OPTIONS] COMMAND [ARGS]...

	Options:
	  --help  Show this message and exit.

	Project Commands:
	  bump         bump the current package version
	  docs         compile documentation for the project
	  integration  run package tests
	  setup        update/create the package_name environment.
	  static       run static typing
	  tests        run package tests

	Global Commands:
	  list  get a list of currently available commands
	  init  Initialize an angreal based project.


5. Now you do your thing and start developing on your software. At some point (hopefully early and often) you'll need to run unit tests.

.. code-block:: bash

  	$: angreal tests

    cachedir: .pytest_cache
    rootdir: /Users/dstorey/Desktop/next_big_thing, inifile:
    plugins: cov-2.6.0
    collected 0 items
    Coverage.py warning: No data was collected. (no-data-collected)


    ---------- coverage: platform darwin, python 3.7.3-final-0 -----------
    Name                         Stmts   Miss  Cover
    ------------------------------------------------
    next_big_thing/__init__.py       0      0   100%
    next_big_thing/cli.py            5      5     0%
    ------------------------------------------------
    TOTAL                            5      5     0%


6. Or maybe you need to compile the documentation you're writing as you go :

.. code-block:: bash

    $: angreal docs


    Creating file source/next_big_thing.cli.rst.
    Creating file source/next_big_thing.rst.
    Running Sphinx v1.8.0
    making output directory...
    building [mo]: targets for 0 po files that are out of date
    building [html]: targets for 3 source files that are out of date
    updating environment: 3 added, 0 changed, 0 removed
    reading sources... [100%] next_big_thing.cli
    /Users/dstorey/Desktop/next_big_thing/docs/source/index.rst:6: WARNING: Problems with "include" directive path:
    InputError: [Errno 2] No such file or directory: 'source/intro.rst'.
    looking for now-outdated files... none found
    pickling environment... done
    checking consistency... /Users/dstorey/Desktop/next_big_thing/docs/source/next_big_thing.rst: WARNING: document isn't included in any toctree
    done
    preparing documents... done
    writing output... [100%] next_big_thing.cli
    generating indices... genindex py-modindex
    writing additional pages... search
    copying static files... done
    copying extra files... done
    dumping search index in English (code: en) ... done
    dumping object inventory... done
    build succeeded, 2 warnings.

    The HTML pages are in build/html.


7. Or maybe you just started developing on a new computer and want to make sure you're setup :

.. code-block:: bash

    $: angreal setup

    Virtual environment next_big_thing updated.


