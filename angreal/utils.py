"""
    angreal.utils
    ~~~~~~~~~~~~~

    utilities for interacting with angreal files and projects
"""

import click
import importlib.util
import os
from pathlib import Path

DEFAULT_FOLDER = '.angreal'


def get_angreal_path(dir=DEFAULT_FOLDER):
    """
    Attempts to find the angreal_tasks file by traversing parent directories until it's found.

    :param dir: location of your projects angreal folder
    :return: path
    """

    current_path = Path(os.getcwd())

    angreal_path = None

    # Look up the tree until we hit the root directory
    paths_to_test = [os.getcwd()] + list(current_path.parents)
    # explicit string conversion to get PosixPath error to knock it off
    paths_to_test = [str(p) for p in paths_to_test]

    for p in paths_to_test:
        test_path = os.path.join(p, dir)
        if os.path.isdir(test_path):
            angreal_path = test_path
            break

    if not angreal_path:
        raise FileNotFoundError("Unable to find angreal_task dir {}.".format(os.path.join(dir)))

    return os.path.abspath(angreal_path)


def import_from_file(file):
    """
    load a module based on a file name

    :param file: The file to be loaded
    :return:
    """
    module_name = os.path.split(file)[-1][:-3]
    spec = importlib.util.spec_from_file_location(module_name, file)
    task = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(task)
    return task


def win(string):
    """
    print a green message for successful

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='green', bold=True))


def warn(string):
    """
    print a yellow message as a warning

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='yellow', bold=True))
    pass


def error(string):
    """
    print a red message as an error

    :param string:
    :return:
    """
    click.echo(click.style(string, fg='red', bold=True))
