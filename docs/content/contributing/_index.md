---
title: Contributing
---

Angreal is hosted on [gitlab](https://gitlab.com/angreal/angreal).

If you have questions, concerns, bug reports, or suggestions please feel
free to open an [issue](https://gitlab.com/angreal/angreal/issues/new),
I\'ll do my best to get it addressed at any time.

If you\'d like to contribute back to angreal\'s code base (or
documentation!) feel free to submit a [merge
request](https://gitlab.com/angreal/angreal/merge_requests/new).

Before submitting a merge request, it would be best if you open a new
issue that outlines what the problem you wish to solve is (and perhaps
see if anyone else is working on it).

**Setting up for development:**

You will need the following software available in your development
environment.

-   [git](https://git-scm.com/)
-   [git-lfs](https://git-lfs.github.com/)
-   [python3.5+](https://www.python.org/downloads/)

It\'s suggested that you also install virtualenv via pip.

-   Clone the source code :
    `git clone git@gitlab.com:angreal/angreal.git`
-   Get a fresh branch : `git checkout -b "branch-name"`
-   Navigate to angreal : `cd angreal`
-   Set up an environment :
    `python -m virtualenv -p python3 .venv/angreal`
-   Activate environment : `source .venv/angreal/bin/activate`
-   Install dependencies : `pip install -r requirements/dev.txt`
