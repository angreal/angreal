"""
    angreal.integrations.virtual_env
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Integration to virtualenv

"""
import functools
import os
import subprocess
import sys
import venv



def venv_required(path,requirements=None):
    """wrap a function in a virtual environment before execution

    Args:
        path (str): The path to the virtual environment (or where the environment
          should be created if it doesn't exist)
        requirements (_type_, optional): A string containing a single module, a
          list of module names, or a string containing a file path. Defaults to None.
    """

    def decorator(f):

        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            initial_sys_prefix = sys.prefix
            venv = VirtualEnv(path=path, now=True,requirements=requirements)
            venv.install_requirements()
            rv = f(*args, **kwargs)
            sys.prefix = initial_sys_prefix
            return rv

        return wrapper

    return decorator


class VirtualEnv(object):

    """
    Interacting with virtual environments from within a currently running script.

    Args:
        path (str): the path to the virtual environment
        requirements ([str,List[str]]), optional): A string containing a single module,
          a list of module names, or a string containing a file path. Defaults to None.
        now (bool, optional): should the environment be created/activated
          on initialization. Defaults to True
    """

    base_path = os.path.expanduser(os.path.join("~", ".venv"))

    @property
    def exists(self):
        """
        Does the virtual environment exist
        """
        return (
            os.path.isdir(self.ensure_directories.bin_path)
            and os.path.isdir(self.ensure_directories.env_dir)
            and os.path.isdir(self.ensure_directories.python_dir)
            and os.path.isdir(self.ensure_directories.inc_path)
            and os.path.isfile(self.ensure_directories.env_exe)
        )

    def __init__(self, path, requirements=None, now=True):
        """
        Initializes the object either creating or activating the named environment.

        """

        self.path = path
        self.devnull = open(os.devnull, "w")

        self.requirements = requirements
        self.env = os.environ.copy()
        self.now = now
        self.ensure_directories = venv.EnvBuilder().ensure_directories(self.path)

        if self.now:
            if not self.exists:
                self._create()
            self._activate()


    def install_requirements(self):
        """
        install requirements the requirements set during initialization.

        :param requirements: path to a requirements file, single requirement,
          or list of requirements
        """

        if not self.requirements:
            return

        args = [self.ensure_directories.env_exe, "-m", "pip", "install"]


        if isinstance(self.requirements, list):
            args = args + self.requirements
        elif os.path.exists(self.requirements):
            args = args + ["-r", self.requirements]
        elif isinstance(self.requirements, str):
            args = args + [self.requirements]
        else:
            raise TypeError(
                "requirements should be one of : file, list, or string got "
                "{type(self.requirements)}"
            )

        rc = subprocess.call(args, stdout=self.devnull, stderr=self.devnull)
        if rc != 0:
            raise EnvironmentError(
                "{} failed to install requirements file.".format(self.path)
            )


    def __str__(self):
        return self.path

    def _create(self):
        """
        Create the described environment.
        """

        builder = venv.EnvBuilder(with_pip=True)
        builder.create(self.path)

    def _activate(self):
        """
        Activate the described environment.
        """
        base = self.ensure_directories.env_dir

        if sys.platform == "win32":  # I'm not sure how stable this is.
            site_packages = os.path.join(base, "lib", "site-packages")
        else:
            site_packages = os.path.join(
                base,
                "lib",
                f"python{sys.version_info[0]}.{sys.version_info[1]}",
                "site-packages",
            )

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
