"""
    angreal.task_helpers
    ~~~~~~~~~~~~~~~~~~~~

    functions that help make writing tasks a little easier

"""
import fnmatch
import os
import shutil


def check_environment(*name):
    """
    Check that a needed environmental variable is set.
    :param name:
    :raises EnvironmentError:
    """
    for n in name:
        if n in os.environ.keys():
            return
        if n.lower() in os.environ.keys():
            return
        if n.upper() in os.environ.keys():
            return
        raise EnvironmentError('The Environemental variable {} is required.'.format(n))

def check_files(*src):
    """
    Check that a needed file is present.
    :param src:
    :raises FileExistsError:
    """
    for s in src:
        if not os.path.isfile(s):
            raise FileNotFoundError('File {} not found.'.format(s))

def copy_files(dst, *src):
    """
    Copy files from source(s) to destination.
    :param dst: destination for all files
    :param src: source(s) of files
    """
    for f in src:
        if os.path.isfile(f):
            shutil.copy(f,dst)

def clean_files(*src):
    """
    Remove a file. This is destructive, use with care.
    :param src: path(s) to file for deletion.
    """
    for f in src:
        if os.path.isfile(f):
            os.unlink(f)


def get_project_source_files(dir):
    """
    Generator that recursively get all python files in a directory.
    :param dir: The directory to start with
    :returns: path to file
    """
    for path, subdirs, files in os.walk(dir):
        for name in files:
            if fnmatch.fnmatch(name, '*.py'):
                yield os.path.join(path,name)
