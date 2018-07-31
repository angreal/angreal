"""
    angreal.base_init
    ~~~~~~~~~~~~~~~~~

    Just a pass through to cookie-cutter

"""


import os
import shutil
import tempfile

import click


from cookiecutter.exceptions import OutputDirExistsException

import angreal
from angreal.cutter import initialize_cutter
from angreal.utils import get_angreal_path, import_from_file

def print_base_help():
    """
    Prints the base help information
    :return:
    """
    with click.Context(base_init) as ctx:
        click.echo(base_init.get_help(ctx))

def print_nested_help(repository):
    """
    Prints a general
    :param repository:
    :return:
    """

    print_base_help()

    try:
        tmp_dir = tempfile.mkdtemp()
        project_path = initialize_cutter(repository,no_input=True,output_dir=tmp_dir)
        os.chdir(project_path)
        mod = import_from_file(os.path.join(get_angreal_path(),'init.py'))

        mod = mod.init
        click.echo("""
    These are the options for the repository ({}) you are attempting to initialize
        """.format(repository))
        with click.Context(mod) as ctx:
            click.echo(mod.get_help(ctx))

        shutil.rmtree(tmp_dir)
    except Exception:
        pass


    exit(0)




@angreal.command(name='init',context_settings=dict(ignore_unknown_options=True),
                 add_help_option=False)
@angreal.argument('repository')
@angreal.argument('init_args', nargs=-1, type=click.UNPROCESSED)
@angreal.option('--no-input', is_flag=True, help='Do not prompt for parameters and only use cookiecutter.json file content')
@angreal.option('--help','-h', is_flag=True, help='Display a help message')
def base_init(repository,init_args,help,no_input=False):
    """
    Initialize an angreal based project.
    """
    if help:
        print_nested_help(repository)
        exit(0)

    try:
        project_path = initialize_cutter(repository,no_input=no_input)
    except OutputDirExistsException:
        exit(-2)
    os.chdir(project_path)
    file = os.path.join(get_angreal_path(), 'init.py')

    try:
        # First try to import the file
        mod = import_from_file(file)
        try:
            # Try to run the "init" function in the task_init file, pass through all of the init_args
            mod.init(init_args)
        except Exception as e:
            # Something happened in the sub init execution
            shutil.rmtree(project_path)
            raise
            exit (-1)

        # Init commands should only be run ONCE
        os.unlink(file)

    except (ImportError, FileNotFoundError):
        # if the file doesn't exist or import fails pass
        shutil.rmtree(project_path)
        exit(-1)

    return




