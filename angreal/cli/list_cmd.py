"""
    angreal.cli.list
    ~~~~~~~~~~~~~~~~

    list available commands in the angreal

"""
from collections import defaultdict
import os

import angreal
from angreal.utils import get_angreal_path,import_from_file

import click

@angreal.command(name='list')
@angreal.argument('nothing', nargs=-1, type=click.UNPROCESSED)
def list_cmd(nothing):
    """
    list available commands
    """
    available_commands = defaultdict(str)

    # Get everything we can in the files adjacent to this file
    for f in os.listdir(os.path.dirname(__file__)):
        f = os.path.join(os.path.dirname(__file__),f)
        if not os.path.basename(f) == '__init__.py':
            try:
                mod = import_from_file(os.path.abspath(f))
            except AttributeError:
                continue
            for m in mod.__dict__:
                if isinstance(mod.__dict__[m],click.core.Command):
                    available_commands[mod.__dict__[m].name] = mod.__dict__[m].help

    #Get everything we can in the .angreal
    try:
        angreal_dir = get_angreal_path()
        for f in os.listdir(angreal_dir):
            f = os.path.join(angreal_dir,f)
            if os.path.basename(f) not in  ['init.py','__init__.py']:
                try:
                    mod = import_from_file(f)
                except AttributeError:
                    continue
                for m in mod.__dict__:
                    if isinstance(mod.__dict__[m],click.core.Command):
                        available_commands[mod.__dict__[m].name] = mod.__dict__[m].help
    except FileNotFoundError:
        pass

    #Format the message and echo it out
    text = "Currently available commands:\n"
    commands = '\n'.join(['{}\t\t{}'.format(m,available_commands[m]) for m in available_commands])
    click.echo(text)
    click.echo(commands)








    pass