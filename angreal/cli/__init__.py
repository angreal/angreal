"""
    angreal.cli
    ~~~~~~~~~~~

    The command line interface for angreal

"""
import os

import click
from collections import defaultdict

import angreal
from angreal import get_angreal_path, import_from_file
from angreal.cli.list_cmd import list_cmd,get_angreal_commands,get_adjacent_commands
from angreal.compat import get_template_version,is_compat

class AngrealCLI(click.MultiCommand):

    def format_commands(self, ctx, formatter):
        rows = []
        for subcommand in self.list_commands(ctx):
            cmd = self.get_command(ctx, subcommand)
            # What is this, the tool lied about a command.  Ignore it
            if cmd is None:
                continue

            help = cmd.short_help or ''
            rows.append((subcommand, help))

        if rows:
            with formatter.section('Project Commands'):
                formatter.write_dl(rows)

    def format_epilog(self, ctx, formatter):
        if self.epilog:
            with formatter.section('Global Commands'):
                formatter.write_dl(self.epilog)


    def list_commands(self,ctx):
        rv = []

        # If we can't find out .angreal , return an empty command list
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

            template_version = get_template_version()
            if template_version:
                if not is_compat(template_version):
                    raise ValueError('This template needs to be run using angreal {}'.format(template_version))

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


def get_base_commands():
    """
    Get base commands included with angreal, return for procesing in the epilog
    :return:
    """
    available_commands = get_adjacent_commands()
    commands = [(m, available_commands[m]) for m in available_commands]
    return commands


def print_version(ctx,param,value):
    """
    print current version of angreal
    """

    click.echo(angreal.__version__)
    exit(0)

@angreal.command(cls=AngrealCLI, epilog=get_base_commands())
def angreal_cmd():
    pass



