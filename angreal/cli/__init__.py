"""
    angreal.cli
    ~~~~~~~~~~~

    The command line interface for angreal

"""
import os

import click

import angreal
from angreal import get_angreal_path, import_from_file
from angreal.cli.list_cmd import list_cmd


class AngrealCLI(click.MultiCommand):

    def list_commands(self,ctx):
        rv = []

        # If we can't find out .angreal , return an empty command listpwd
        try:
            angreal_path = get_angreal_path()
        except FileNotFoundError:
            return []

        #Otherwise, get all the 'task' files available
        for file in os.listdir(angreal_path):
            if file.endswith('.py') and file.startswith('task_'):
                rv.append(file[5:-3])
        rv.sort()
        return rv


    def get_command(self, ctx, name):
        if name == 'init':
            mod = __import__('angreal.cli.base_init',
                             None, None, ['base_init'])
            return mod.base_init

        if name == 'list':
            mod = __import__('angreal.cli.list_cmd',
                             None,None, ['list_cmd'])
            return mod.list_cmd


        try:
            file = os.path.join(get_angreal_path(),'task_{}.py'.format(name))
        except FileNotFoundError:
            click.echo("This doesn't appear to be an angreal!\n")
            list_cmd()
            exit(-1)
        try :
            mod = import_from_file(file)
        except FileNotFoundError:
            click.echo("That sub command doesn't appear to be supported by this angreal!\n")
            list_cmd()
            exit(-1)
        return mod.angreal_cmd


@angreal.command(cls=AngrealCLI)
def angreal_cmd():
    return



