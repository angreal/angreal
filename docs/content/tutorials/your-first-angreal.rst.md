---
title: Your First Angreal
---

{{% notice note %}}This project is available as part of the git repository[here](https://gitlab.com/dylanbstorey/angreal/tree/master/example).{{% /notice %}}

This a very simple project for taking meeting minutes !

We have some basic requirements for this template :

-   on creation a folder with the meeting name will be created
-   on creation a file that introduces us to the reason the meetings
    exist as an initial README.
-   a task to take minutes using an editor

## Directory Template

Based on the above planned requirements we'll need files and folders created as follows : 

```bash
meeting_notes
├── angreal.toml
└── {{ name }}
    ├── .angreal
    │   ├── init.py
    │   └── task_take_notes.py
    └── README.md
```

## Init Script
{{% notice info %}}
**Optional:** In this very trivial example, not much happens after the folder structure is created so the init isn't required. 
{{% /notice %}}

```python
def init():
    print("Initializing {{ name }} !")
    return
```

## Create our Task to Take Minutes

{{% notice info %}}
Angreal tasks must be a function in a python file that starts with `task_`.
{{% /notice %}}


```python
import angreal

import datetime
import os
import subprocess
import tempfile


@angreal.command(name='take-notes', about='take notes for our meeting')
def angreal_cmd(now):
    """
    create a file for taking minutes
    """
    file_name = datetime.datetime.today().strftime('%Y-%m-%d-%H-%M')

    # We're going to assume that you're running on ubuntu 
    # which has a binary called "editor" that will launch your 
    # default terminal editor. If you need something else - set the environment
    # variable "EDITOR" to the appropriate command 
    editor = os.environ.get('EDITOR','editor')

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
```

## Using our Angreal
