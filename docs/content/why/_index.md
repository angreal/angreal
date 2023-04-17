---
title: Why Angreal ?
weight: 10
---

## Why use this tool?


Imagine you make a practice of writing lots of software packages either
as an individual or as part of a team. Every time you have to create the
directory and file structure, you probably do this by hand. You probably
do it just a little differently each time, making small inconsistencies
on each project. Now imagine you also have to work on multiple software
packages and each one is set up a little differently, with slightly
different way of doing basic tasks like running unit tests. You would
spend at least some amount of resources : remembering the differences
between each project or fixing little mistakes between them.

The whole point of angreal is to allow you to forget how to set up a
project AND how to interact with it. By using templates for both project
structure AND administration, you don\'t have to remember how to do a
task - just that it can be done.

## How do I use them ?

You interact with an angreal in two steps :

1.  **Initialization** : Initializing an angreal means rendering a
    described template into a usable form. Templates can exist in one of
    three locations : local directories, remote git repositories, or
    pypi modules. Where an angreal template is distributed from is
    decided by whomever made that particular template.

```bash
# initialize and render the template angreal at the remote URI
$: angreal init https://github.com/angreal/python.git
# If you've used this template before you simply need to ...
$: angreal init python
```

2.  **Execution** : Executing an angreal task through the `angreal`
    command line interface. This can only happen after the template has
    been rendered.

```bash
# From within an angreal based template
$: angreal <command to run>
```

## A Quick Example

Let\'s say you\'re starting a new software project and plan on using
[python 3](www.python.org). You also want to make sure that you\'re
following good practices and have documentation, testing, and type
hinting as part of the project.

You identify that the python3 angreal fits your needs well and choose to
use it.

1.  You initialize the project with :

```bash
$: angreal init https://github.com/angreal/python.git
```

2.  You provide values for template variables with the interactive CLI :

```bash
author_name? ["Your name"]
Dylan Storey
author_email? ["Your email address (eq. you@example.com)"]
dylan.storey@gmai.com
github_username? ["yourname"]
dylanbstorey
project_name? ["Name of the project (will be shown e.g. as the title in the readme)"]
Angreal Python Demo
project_slug? ["angreal-python-demo"] #enter accepts the provided devault value

package_name? ["angreal_python_demo"]

project_short_description? ["A short description of the project"]
A demo project
```

3.  Angreal creates a project for you, using your provided variables to
    fill in some of the blanks.

```bash
cd angreal-python-demo && tree
.
├── angreal_python_demo
│   ├── __init__.py
│   └── __main__.py
├── CHANGELOG.md
├── conftest.py
├── LICENCE
├── MANIFEST.in
├── pyproject.toml
├── README.md
├── setup.cfg
├── setup.py
└── tests
    ├── integration
    │   └── __init__.py
    └── unit
        └── __init__.py

4 directories, 12 files
```

4.  Next lets find out what tasks come with the template

```bash
$: angreal                                                                                                                                       ─╯
angreal 2.0.0-rc.1

USAGE:
    angreal [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    verbose level, (may be used multiple times for more verbosity)
    -V, --version    Print version information

SUBCOMMANDS:
    build        build your project for distribution
    clean        cleans out generated cruft
    dev-setup    setup a development environment
    help         Print this message or the help of the given subcommand(s)
    init         Initialize an Angreal template from source.
    run-tests    run our test suite. default is unit tests only

```

5.  Now you do your thing and start developing on your software. At some
    point (hopefully early and often) you\'ll need to run unit tests.

```bash
$: angreal run-tests --open

================================================================ test session starts ================================================================platform linux -- Python 3.8.10, pytest-7.2.0, pluggy-1.0.0 -- /home/dstorey/.venvs/angreal-2/bin/python3
cachedir: .pytest_cache
rootdir: /home/dstorey/angreal-python-demo, configfile: setup.cfg
plugins: cov-4.0.0
collected 0 items
/home/dstorey/.venvs/angreal-2/lib/python3.8/site-packages/coverage/control.py:836: CoverageWarning: No data was collected. (no-data-collected)
  self._warn("No data was collected.", slug="no-data-collected")


---------- coverage: platform linux, python 3.8.10-final-0 -----------
Name                              Stmts   Miss  Cover
-----------------------------------------------------
angreal_python_demo/__init__.py       1      1     0%
angreal_python_demo/__main__.py       2      2     0%
-----------------------------------------------------
TOTAL                                 3      3     0%
Coverage HTML written to dir htmlcov

========
```


6.  Or maybe you just started developing on a new computer and want to
    make sure you\'re setup :

```bash
$: pip install angreal
$: git clone https://github.com/dylanbstorey/angreal-test-project
$: cd angreal-test-project
$: angreal dev-setup
```
