---
title: git
---

# git

angreal.integrations.git
~~~~~~~~~~~~~~~~~~~~~~~~

programmatic access to git

## Classes

### GitException

GitException

#### Methods

##### __init__

```python
__init__(self, message)
```

### Git

Hyper light weight wrapper for git

**git_path**: path to the git file
**working_dir**: the working directory to work from

#### Methods

##### __init__

```python
__init__(self, git_path, working_dir)
```

Constructor for Git

##### __call__

```python
__call__(self, command, *args, **kwargs)
```

**command**: the sub command to be run
    **args**: the arguments the command needs
    **kwargs**: the options for the command
    :return tuple: return_code, stderr, stdout from the completed command

##### __getattr__

```python
__getattr__(self, name, *args, **kwargs)
```

Make calls to git sub commands via method calls.

    *Example*:
```python

        git = Git()
        git.add('.')
        git.clone('gitlab.git')

    **name**: the subcommand you wish to call
    **args**: mandatory parameters
    **kwargs**: optional arguments (flags)
    **Returns**:
