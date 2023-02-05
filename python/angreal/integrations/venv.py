"""
    angreal.integrations.virtual_env
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Integration to virtualenv

"""
import logging
import functools
import os
import subprocess
import sys
import venv
import site

from shutil import which


def venv_required(path):
    """
    Ensure that you're operating in the "correct" environment via sys.prefix. Will create and activate the environment
    if it doesn't exist.

    :param name: The name of the environment
    :param requirements_file: full path the requirements file for activation
    :return:
    """
    initial_sys_prefix = sys.prefix
    venv = VirtualEnv(path=path, now=False)

    if venv.exists:
        venv._activate()
    else:
        raise EnvironmentError(
            "virtual environment does not exist at {}".format(venv.path)
        )

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
    :param now: should the environment be created/activated on initialization
    :param location: where the venv should be located. defaults to `.venv`.
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

        if self.requirements:
            self.install_requirements(self.requirements)

    def install_requirements(self, requirements):
        """
        install requirements from a file

        :param requirements: path to a requirements file
        """
        args = [
            self.ensure_directories.env_exe,
            "-m",
            "pip",
            "install",
            "-r",
            requirements,
        ]

        rc = subprocess.call(args, stdout=self.devnull, stderr=self.devnull)
        if rc != 0:
            raise EnvironmentError(
                "{} failed to install requirements file.".format(self.name)
            )

    def __str__(self):
        return self.path

    def _create(self):
        """
        create a virtual environment from the current settings
        :return:
        """

        builder = venv.EnvBuilder(with_pip=True)
        builder.create(self.path)

    def _activate(self):
        """
        activate the current virtual environment (shamelessly lifted from activate_this.py)
        :return:
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
