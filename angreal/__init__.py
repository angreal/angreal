import os
import click
from angreal.utils import get_angreal_path, import_from_file
from angreal.decorators import *
import importlib




class AngrealCLI(click.MultiCommand):

    def list_commands(self,ctx):
        rv = list()
        angreal_path = get_angreal_path()
        for file in os.listdir(angreal_path):
            if file.endswith('.py') and file.startswith('task_'):
                rv.append(file[5:-3])
        rv.sort()
        return rv


    def get_command(self, ctx, name):
        file = os.path.join(get_angreal_path(),'task_{}.py'.format(name))
        mod = import_from_file(file)
        return mod.angreal_cmd


importlib.import_module('click')

__all__ = [
    'AngrealCLI',

    # Click Decorators
    'pass_context', 'pass_obj', 'make_pass_decorator', 'command', 'group',
    'argument', 'option', 'confirmation_option', 'password_option',
    'version_option', 'help_option',

    # Click Utilities
    'echo', 'get_binary_stream', 'get_text_stream', 'open_file',
    'format_filename', 'get_app_dir', 'get_os_args',

]