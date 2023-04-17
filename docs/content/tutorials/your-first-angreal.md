---
title: Your First Angreal
weight: 10
---

{{% notice note %}}This project is available under the [example folder of the git repository](https://gitlab.com/dylanbstorey/angreal/tree/master/example).{{% /notice %}}

This a very simple project for taking meeting minutes !

We have some basic requirements for this template :

-   on creation a folder with the meeting name will be created
-   on creation a file that introduces us to the reason the meetings
    exist as an initial README.
-   a task to take minutes using an editor

## Template Layout

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
## `angreal.toml`

The `angreal.toml` file tells angreal what variables it needs to template and provides default values for them.
Our template will have the following variables:
- name
- cadence
- standing_agenda

```toml
name="another_meeting"
cadence="weekly"
standing_agenda="Complaints"
```

## `README.md`

The `README.md` is just meant to be a highlevel description of the meeting so you can remember why you're there every week.

```markdown
# {{ name }}


## Cadence

{{ cadence }}


## Standing Agenda

{{ standing_agenda }}
```


## `init.py`
{{% notice info %}}
**Optional:** In this very trivial example, not much happens after the folder structure is created so the init isn't required.
{{% /notice %}}

```python
def init():
    print("Initializing {{ name }} !")
    return
```

## `task_take_notes.py`

{{% notice info %}}
Angreal tasks must be a function in a python file that starts with `task_` in the `.angreal` folder.
{{% /notice %}}


```python
import angreal

import datetime
import os
import subprocess
import tempfile


@angreal.command(name='take-notes', about='Take notes for our meeting')
@angreal.argument(name='now',long='now', takes_value=False)
def angreal_cmd(now=False):
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
A brief explanation of this code:
    - import angreal and other libraries
    - decorate a function with the `command`
    - we decorate the same function with an `argument`

The function itself:
- determines the current date/time
- tries to get an EDITOR variable from the environment, falling back to the `editor` command from Ubuntu
- if you pass the `--now` argument, opens a temporary file using your editor
- saves the notes taken to a file with the date and time the minutes were started.

## Using our Angreal

1. Initialize our template.
```bash
$ angreal init docs/content/tutorials/meeting_notes

cadence? ["weekly"]
>
name? ["another_meeting"]
> Hall of the Tower
standing_agenda? ["Complaints"]
> Discussing embroidery and fine turned calves

Initializing Hall of the Tower !
```
```bash
$ tree 'Hall of the Tower'
Hall of the Tower
└── README.md
```

```bash
$ cat Hall\ of\ the\ Tower/README.md                                                                                                            ─╯
# Hall of the Tower


## Cadence

Weekly


## Standing Agenda

Discussing embroidery and fine turned calves.

```

2. What commands do i have access to ?

```bash
$ cd 'Hall of the Tower'
$ angreal

angreal 2.0.0-rc.1

USAGE:
    angreal <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help          Print this message or the help of the given subcommand(s)
    init          Initialize an Angreal template from source.
    take-notes    Take notes for our meeting

```

3. How do i use `take-notes` ?

```bash
$ angreal take-notes --help

take-notes

    create a file for taking minutes


USAGE:
    take-notes [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

        --now
            open editor immediately
```

Lets take some minutes, right now

```bash
$ export EDITOR='vim'
$ angreal take-minutes --now
```

This will open an editor (vim if you set the `EDITOR` variable) write a note.

```
$ ls                                                                                                                                            ─╯
2023-01-19-09-19.md  README.md
```
