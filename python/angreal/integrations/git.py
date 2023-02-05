"""

    angreal.integrations.git
    ~~~~~~~~~~~~~~~~~~~~~~~~

    programmatic access to git

"""
import os
import subprocess
from shutil import which


class GitException(Exception):
    """
    GitException
    """

    def __init__(self, message):
        super().__init__(message)


class Git(object):
    """
    Hyper light weight wrapper for git

    :param git_path: path to the git file
    :param working_dir: the working directory to work from
    """

    def __init__(self, git_path=None, working_dir=None):
        """Constructor for Git"""

        if not git_path:
            git_path = which("git")
        self.git_path = git_path

        if not working_dir:
            self.working_dir = os.path.abspath(os.getcwd())
        else:
            if not os.path.isdir(working_dir):
                raise FileNotFoundError()
            self.working_dir = os.path.abspath(working_dir)

        try:
            assert os.path.isfile(self.git_path)
        except (TypeError, AssertionError):
            raise OSError("git not in path")

    def __call__(self, command, *args, **kwargs):
        """
        :param command: the sub command to be run
        :param args: the arguments the command needs
        :param kwargs: the options for the command
        :return tuple: return_code, stderr, stdout from the completed command
        """

        # unpack a command (git init --this=that -t=7 repo)
        system_call = (
            ("git", command)
            + tuple(
                ("--{0}={1}".format(k, v) if len(k) > 1 else "-{0} {1}".format(k, v))
                for k, v in kwargs.items()
            )
            + args
        )

        process_return = subprocess.run(
            system_call,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=self.working_dir,
        )

        if process_return.returncode != 0:
            message = "git non-zero exit status ({2}): {0} {1}".format(
                process_return.args, process_return.stderr, process_return.returncode
            )
            raise GitException(message)

        return process_return.returncode, process_return.stderr, process_return.stdout

    def __getattr__(self, name, *args, **kwargs):
        """
        Make calls to git sub commands via method calls.

        i.e. ::

            git = Git()
            git.add('.')
            git.clone('gitlab.git')



        :param name: the subcommand you wish to call
        :param args: mandatory parameters
        :param kwargs: optional arguments (flags)
        :return:
        """
        return lambda *args, **kwargs: self(name, *args, **kwargs)
