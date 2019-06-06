"""
    angreal.base_init
    ~~~~~~~~~~~~~~~~~

    The initialization command for angreals is here, handles initial clone/templating and executing
    template specific initialization steps.

"""


import os
import shutil
import tempfile

import click
from cookiecutter.exceptions import OutputDirExistsException

import angreal
from angreal.compat import get_template_version, is_compat
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

    try:
        tmp_dir = tempfile.mkdtemp()
        project_path = initialize_cutter(repository, no_input=True, output_dir=tmp_dir)
        os.chdir(project_path)
        mod = import_from_file(os.path.join(get_angreal_path(), 'init.py'))

        mod = mod.init
        click.echo("\nThese are the options for the repository ({}) you are attempting to initialize\n".format(repository))
        with click.Context(mod) as ctx:
            click.echo(mod.get_help(ctx))

        shutil.rmtree(tmp_dir)

    except Exception:
        click.echo("Something happened while fetching this temlpates help information, you should check the source for more information")

    exit(0)


@angreal.command(name='init', context_settings=dict(ignore_unknown_options=True),
                 add_help_option=False)
@angreal.argument('repository')
@angreal.argument('init_args', nargs=-1, type=click.UNPROCESSED)
@angreal.option('--no-input', is_flag=True, help='Do not prompt for parameters and only use cookiecutter.json file content')
@angreal.option('--help', '-h', is_flag=True, help='Display a help message')
def base_init(repository, init_args, help, no_input=False):
    """
    Initialize an angreal based project.
    """
    if help:
        print_nested_help(repository)
        exit(0)

    try:
        project_path = initialize_cutter(repository, no_input=no_input, replay=True)
    except OutputDirExistsException:
        exit("Output directory exists")

    os.chdir(project_path)

    # Check compat of executing angreal binary against .angreal/VERSION
    template_version = get_template_version()
    if template_version:
        if not is_compat(template_version):
            raise ValueError('This template needs to be run using angreal {}'.format(template_version))

    file = os.path.join(get_angreal_path(), 'init.py')

    try:
        # First try to import the file
        mod = import_from_file(file)
        try:
            # Try to run the "init" function in the task_init file, pass through all of the init_args
            mod.init(init_args)

        except AttributeError as e:
            pass

        except Exception as e:
            # Something happened in the sub init execution
            shutil.rmtree(project_path)
            exit("Exiting at line 103:\n{}".format(e))
        # Init commands should only be run ONCE
        os.unlink(file)

    except FileNotFoundError:
        # No init.py means no special startup instructions.
        pass

    except (ImportError, AttributeError):
        # if the init function doesn't exist or import fails pass
        response = input("Import error on init.py, do you want to keep the generated project? [y/n]")
        if response == 'n':
            shutil.rmtree(project_path)

    return
