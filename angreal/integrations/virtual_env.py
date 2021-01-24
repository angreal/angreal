"""
    angreal.integrations.virtual_env
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Integration to virtualenv

"""

import functools
import os
import subprocess
import sys
from virtualenv import cli_run
from distutils.spawn import find_executable


def venv_required(name):
    """
    Ensure that you're operating in the "correct" environment via sys.prefix. Will create and activate the environment
    if it doesn't exist.

    :param name: The name of the environment
    :param requirements_file: full path the requirements file for activation
    :return:
    """

    initial_sys_prefix = sys.prefix
    venv = VirtualEnv(name=name,
               now=False)

    # more checks against current python + os.environ.VIRTUAL_ENV

    if venv.exists:
        venv._activate()
    else:
        raise EnvironmentError('virtual environment {} does not exist at {}'.format(venv.name, venv.path))


    def decorator(f):
        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            rv = f(*args, **kwargs)
            sys.prefix = initial_sys_prefix
            return rv
        return wrapper
    return decorator


class VirtualEnv(object):
    """
    Interacting with virtual environments from within a currently running script.

    :param name: the name of the virtual environment
    :param python: the path (or basename) of the python executable to use for an interpreter
    :param requirements: a requirements file to use for creation
    :param now: should the environment be created on init
    """

    base_path = os.path.expanduser(os.path.join('~', '.venv'))

    @property
    def path(self):
        """
        What's the path to the virtual environment
        :return:
        """
        os.makedirs(self.base_path, exist_ok=True)
        return os.path.join(self.base_path, self.name)

    @property
    def active(self):
        """
        determines if the current object is active
        :return:
        """
        return os.path.basename(sys.prefix) == self.name
        return

    @property
    def bin(self):
        return os.path.join(self.path, 'bin')

    @property
    def pip(self):
        return os.path.join(self.bin, 'pip')

    @property
    def python_exe(self):
        return os.path.join(self.bin, 'python')

    @property
    def lib(self):
        return os.path.join(self.path, 'lib')

    @property
    def exists(self):
        """
        determine if the current environment exists
        :return:
        """

        return (os.path.isdir(self.lib) and
                os.path.isdir(self.bin) and
                os.path.isfile(self.python_exe) and
                os.path.isfile(self.pip))

    def __init__(self, name, python=None, requirements=None, now=True):
        """
        Initializes the object either creating or activating the named environment.

        """
        self.name = name

        if not python:
            python = 'python'

        self.python = python
        self.devnull = open(os.devnull, 'w')

        self.requirements = requirements
        self.env = os.environ.copy()

        if now:
            self.activate_or_create()

        if self.requirements:
            self.install_requirements(self.requirements)

    def install_requirements(self, requirements):
        """
        install requirements from a file

        :param requirements: path to a requirements file
        """
        args = ['python', '-m', 'pip', 'install', '-r', requirements]

        rc = subprocess.call(args, stdout=self.devnull, stderr=self.devnull)

        if rc != 0:
            raise EnvironmentError('{} failed to install requirements file.'.format(self.name))

    def __str__(self):
        return self.path

    def activate_or_create(self):
        if not self.exists:
            self._create()
        if not self.active:
            self._activate()

    def _create(self):
        """
        create a virtual environment from the current settings
        :return:
        """

        cli_run(['-p', self.python, self.path])


    def _activate(self):
        """
        activate the current virtual environment
        :return:
        """
        
        if not os.path.isdir(self.path):
            raise FileNotFoundError('No Virtual Environment found for {}'.format(self.name))
            
            

        exec(open(self.activate_script,'r').read(), dict(__file__=self.activate_script))
        
        pass

    @property
    def activate_script(self):
        """
        The path to this environments activate_this.py
        """
        return os.path.join(self.path, 'bin', 'activate_this.py')

