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


def venv_required(name):
    """
    Ensure that you're operating in the "correct" environment via sys.prefix.

    - sys.prefix =~ name

    :param name: The name of the environment
    :return:
    """

    #more checks against current python + os.environ.VIRTUAL_ENV
    if not os.path.basename(sys.prefix) == name :
        raise ValueError('virtualenv {} is not activated (active: {})'.format(name,os.path.basename(sys.prefix)))

    def decorator(f):
        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            return f(*args,**kwargs)
        return wrapper
    return decorator


class VirtualEnv(object):

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
    def exists(self):
        """
        determine if the current environment exists
        :return:
        """
        lib = os.path.join(self.path,'lib')
        include = os.path.join(self.path,'include')
        bin = os.path.join(self.path,'bin')
        python = os.path.join(bin,'python')
        pip = os.path.join(bin,'pip')

        return ( os.path.isdir(lib) and
                 os.path.isdir(include) and
                 os.path.isdir(bin) and
                 os.path.isfile(python) and
                 os.path.isfile(pip) )


    def __init__(self,name, python=None,requirements=None,now=True):
        self.name = name

        if not python:
            python='python'

        self.python=python


        self.requirements=requirements
        self.env = os.environ.copy()

        if now:
            self.activate_or_create()


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

        proc = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
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

