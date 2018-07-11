import os

import click

import angreal
from angreal import get_angreal_path, import_from_file


class AngrealCLI(click.MultiCommand):

    def list_commands(self,ctx):
        rv = list()

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

        file = os.path.join(get_angreal_path(),'task_{}.py'.format(name))
        mod = import_from_file(file)
        return mod.angreal_cmd


@angreal.command(cls=AngrealCLI)
def angreal_cmd():
    return



