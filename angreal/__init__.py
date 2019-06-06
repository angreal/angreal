"""
    angreal
    ~~~~~~~


"""
import importlib
import os
import click

from angreal.decorators import *
from angreal.utils import get_angreal_path, import_from_file, win, warn, error
from angreal.integrations.gh import GitHub
from angreal.integrations.gl import GitLab
from angreal.integrations.virtual_env import VirtualEnv
from angreal.integrations.doit import make_doit_task, doit_task, run_doit_tasks

from angreal.replay import Replay

importlib.import_module('click')


__version__ = open(os.path.join(os.path.dirname(__file__), 'VERSION')).read().strip()


__all__ = [
    # Click Decorators
    'pass_context', 'pass_obj', 'make_pass_decorator', 'command', 'group',
    'argument', 'option', 'confirmation_option', 'password_option',
    'version_option', 'help_option',

    # Click Utilities
    'echo', 'get_binary_stream', 'get_text_stream', 'open_file',
    'format_filename', 'get_app_dir', 'get_os_args',

    # Angreal Utilities
    'get_angreal_path', 'import_from_file', 'win', 'warn', 'error',

    # Core classes
    'GitLab', 'GitHub', 'VirtualEnv', 'Replay',

    # Virtual Environmets
    'venv_required',

    # Doit functions
    'doit_task', 'make_doit_task', 'run_doit_tasks'
]
