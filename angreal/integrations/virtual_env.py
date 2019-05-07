"""
    angreal.integrations.virtual_env
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Integration to virtualenv

"""

from distutils.spawn import find_executable
import functools
import os
import sys
import subprocess


def venv_required(name,requirements_file=None):
    """
    Ensure that you're operating in the "correct" environment via sys.prefix. Will create and activate the environment
    if it doesn't exist.

    :param name: The name of the environment
    :param requirements_file: full path the requirements file for activation
    :return:
    """

    initial_sys_prefix = sys.prefix
    venv = VirtualEnv(name=name,requirements=requirements_file)
    #more checks against current python + os.environ.VIRTUAL_ENV
    if not os.path.basename(sys.prefix) == name :
        raise ValueError('virtualenv {} is not activated (active: {})'.format(name,os.path.basename(sys.prefix)))

    def decorator(f):
        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            rv =  f(*args,**kwargs)
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

    base_path = os.path.expanduser(os.path.join('~','.venv'))

    @property
    def path(self):
        """
        What's the path to the virtual environment
        :return:
        """
        os.makedirs(self.base_path,exist_ok=True)
        return os.path.join(self.base_path,self.name)

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
        return os.path.join(self.path,'bin')

    @property
    def pip(self):
        return os.path.join(self.bin,'pip')

    @property
    def python_exe(self):
        return os.path.join(self.bin,'python')

    @property
    def lib(self):
        return os.path.join(self.path,'lib')

    @property
    def include(self):
        return os.path.join(self.path,'include')

    @property
    def exists(self):
        """
        determine if the current environment exists
        :return:
        """

        return ( os.path.isdir(self.lib) and
                 os.path.isdir(self.include) and
                 os.path.isdir(self.bin) and
                 os.path.isfile(self.python_exe) and
                 os.path.isfile(self.pip) )


    def __init__(self,name, python=None,requirements=None,now=True):
        """
        Initializes the object either creating or activating the named environment.


        """
        self.name = name

        if not python:
            python='python'

        self.python=python
        self.devnull = open(os.devnull,'w')

        self.requirements=requirements
        self.env = os.environ.copy()

        if now:
            self.activate_or_create()

        if self.requirements:
            self.install_requirements(self.requirements)


    def install_requirements(self,requirements):
        """
        install requirements from a file

        :param requirements: path to a requirements file
        """
        args = [self.pip, 'install', '-r', requirements]


        proc = subprocess.Popen(args, stdout=self.devnull, stderr=self.devnull)
        output,error = proc.communicate()

        print(' '.join(args),file=sys.stderr)

        if proc.returncode:
            raise EnvironmentError('{} failed with the following information :\n{}\n{} '.format(self.name, proc.returncode, output))


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

        if find_executable(self.python):
            self.python = find_executable(self.python)
        else :
            raise ValueError("Unable to find '{}' in $PATH".format(python))

        args = ['virtualenv']

        if self.python :
            args.extend(['-p', self.python])

        args.append(self.path)

        proc = subprocess.Popen(args, stdout=self.devnull, stderr=self.devnull)
        output,error = proc.communicate()

        if proc.returncode:
            raise EnvironmentError('{} failed with the following information :\n{5:}\n{5:}'.format(self.name, proc.returncode, output))



    def _activate(self):
        """
        activate the current virtual environment
        :return:
        """
        if not os.path.isdir(self.path):
            raise FileNotFoundError('No Virtual Environment found for {}'.format(name))

        with open(self.activate_script,'w') as f:
            print(self.activate_this_text, file=f)

        exec( self.activate_this_text, dict(__file__=self.activate_script))
        pass


    @property
    def activate_script(self):
        """
        The path to this environments activate_this.py
        """
        return os.path.join(self.path,'bin','activate_this.py')

    @property
    def activate_this_text(self):
        """
        The text that belongs in the activate_this.py file.
        """
        return '''
"""By using execfile(this_file, dict(__file__=this_file)) you will
activate this virtualenv environment.

This can be used when you must use an existing Python interpreter, not
the virtualenv bin/python
"""

try:
    __file__
except NameError:
    raise AssertionError(
        "You must run this like execfile('path/to/activate_this.py', dict(__file__='path/to/activate_this.py'))")
import sys
import os

old_os_path = os.environ.get('PATH', '')
os.environ['PATH'] = os.path.dirname(os.path.abspath(__file__)) + os.pathsep + old_os_path
base = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if sys.platform == 'win32':
    site_packages = os.path.join(base, 'Lib', 'site-packages')
else:
    site_packages = os.path.join(base, 'lib', 'python%s' % sys.version[:3], 'site-packages')
prev_sys_path = list(sys.path)
import site
site.addsitedir(site_packages)
sys.real_prefix = sys.prefix
sys.prefix = base
# Move the added items to the front of the path:
new_sys_path = []
for item in list(sys.path):
    if item not in prev_sys_path:
        new_sys_path.append(item)
        sys.path.remove(item)
sys.path[:0] = new_sys_path
    '''

