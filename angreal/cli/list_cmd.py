"""
    angreal.cli.list
    ~~~~~~~~~~~~~~~~

    default command lists available commands in the angreal

"""
import os
from collections import defaultdict

import click

import angreal
from angreal.utils import get_angreal_path, import_from_file


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
                except ImportError:
                    continue
                for m in mod.__dict__:
                    if isinstance(mod.__dict__[m], click.core.Command):
                        name = os.path.basename(f)
                        name = name[5:]
                        name = name[:-3]
                        available_commands[name] = mod.__dict__[m].help
    except FileNotFoundError:
        pass

    return available_commands


@angreal.command(name='list')
@angreal.argument('nothing', nargs=-1, type=click.UNPROCESSED)
def list_cmd(nothing):
    """
    get a list of currently available commands
    """

    adjacent_commands = get_adjacent_commands()
    angreal_commands = get_angreal_commands()

    # Format the message and echo it out

    text = "Base Angreal Commands :"
    commands = '\n'.join(['{0:10}\t\t{1}'.format(m, adjacent_commands[m]) for m in adjacent_commands])
    click.echo(text)
    click.echo(commands)

    if angreal_commands:
        text = "\nCommands in Current Angreal:"
        commands = '\n'.join(['{0:10}\t\t{1}'.format(m, angreal_commands[m]) for m in angreal_commands])
        click.echo(text)
        click.echo(commands)
    exit(0)
