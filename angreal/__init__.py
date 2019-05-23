"""
    angreal
    ~~~~~~~


"""
import importlib
import os

from angreal.decorators import *
from angreal.utils import get_angreal_path, import_from_file

importlib.import_module('click')

__all__ = [
    # Click Decorators
    'pass_context', 'pass_obj', 'make_pass_decorator', 'command', 'group',
    'argument', 'option', 'confirmation_option', 'password_option',
    'version_option', 'help_option',

    # Click Utilities
    'echo', 'get_binary_stream', 'get_text_stream', 'open_file',
    'format_filename', 'get_app_dir', 'get_os_args',

]

__version__ = open( os.path.join( os.path.dirname(__file__), 'VERSION')).read().strip()


import click

def win(string):
    """
    print a green message for successful

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='green', bold=True))
    pass

def warn(string):
    """
    print a yellow message as a warning

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='yellow',bold=True))
    pass

def error(string):
    """
    print a red message as an error

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='red',bold=True))