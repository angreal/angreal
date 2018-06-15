"""
    angreal.base_init
    ~~~~~~~~~~~~~~~~~

    Just a pass through to cookie-cutter

"""



import angreal
import os
import sys
from angreal.cutter import initialize_cutter
from angreal.utils import get_angreal_path,import_from_file
import click

import inspect
from click.testing import CliRunner


def angreal_init(repository,init_args):
    project_path = initialize_cutter(repository)
    os.chdir(project_path)
    file = os.path.join(get_angreal_path(), 'init.py')

    try:
        # First try to import the file
        mod = import_from_file(file)

        try:
            # Try to run the "init" function in the task_init file, pass through all of the init_args
            runner = CliRunner()
            result = runner.invoke(mod.init, init_args)
            print(result.output)
        except Exception:
            # Failures should raise immediately
            raise

        # Init commands should only be run ONCE
        os.unlink(file)

    except (ImportError, FileNotFoundError):
        # if the file doesn't exist or import fails pass
        pass

    return



def print_nested_help():
    with click.Context(base_init) as ctx:
        click.echo(base_init.get_help(ctx))





@angreal.command(context_settings=dict(ignore_unknown_options=True),
                 add_help_option=False)
@angreal.argument('repository')
@angreal.argument('init_args', nargs=-1, type=click.UNPROCESSED)
@angreal.option('--help',is_flag=True)
def base_init(repository,init_args,help):
    """
    Initialize an angreal based project.
    """
    if help:
        print_nested_help()
        exit(0)

    angreal_init(repository,init_args)




