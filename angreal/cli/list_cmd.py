"""
    angreal.cli.list
    ~~~~~~~~~~~~~~~~

    default command lists available commands in the angreal

"""
from collections import defaultdict
import os

import angreal
from angreal.utils import get_angreal_path,import_from_file

import click

def get_adjacent_commands():
    """
    Get commands that are adjacent to angreal's main entry point
    :return:
    """
    available_commands = defaultdict(str)
    # Get everything we can in the files adjacent to this file
    for f in os.listdir(os.path.dirname(__file__)):
        f = os.path.join(os.path.dirname(__file__), f)
        if not os.path.basename(f) == '__init__.py':
            try:
                mod = import_from_file(os.path.abspath(f))
            except AttributeError:
                continue
            for m in mod.__dict__:
                if isinstance(mod.__dict__[m], click.core.Command):
                    available_commands[mod.__dict__[m].name] = mod.__dict__[m].help

    return available_commands


def get_angreal_commands():
    """
    Get everything from the .angreal folder
    :return:
    """
    available_commands = defaultdict(str)
    # Get everything we can in the .angreal
    try:
        angreal_dir = get_angreal_path()
        for f in os.listdir(angreal_dir):
            f = os.path.join(angreal_dir, f)
            if os.path.basename(f) not in ['init.py', '__init__.py']:
                try:
                    mod = import_from_file(f)
                except AttributeError:
                    continue
                for m in mod.__dict__:
                    if isinstance(mod.__dict__[m], click.core.Command):
                        available_commands[mod.__dict__[m].name] = mod.__dict__[m].help
    except FileNotFoundError:
        pass

    return available_commands


@angreal.command(name='list')
@angreal.argument('nothing', nargs=-1, type=click.UNPROCESSED)
def list_cmd(nothing):
    """
    get a list of currently available commands
    """

    available_commands = {**get_adjacent_commands(), **get_angreal_commands()}
    #Format the message and echo it out
    text = "Currently available commands:\n"
    commands = '\n'.join(['{0:10}\t\t{1}'.format(m,available_commands[m]) for m in available_commands])
    click.echo(text)
    click.echo(commands)
    exit(0)
    pass