import os
from pathlib import Path
from doit import loader
import inspect

DEFAULT_FILE = os.path.join('.angreal', 'angreal_tasks.py')

DOIT_CMDS = ['list','info','help','auto','clean','forget','ignore']

def get_task_names(file=DEFAULT_FILE):
    """
    Get a list of task names in the angreal file
    :param file:
    :return:
    """

    module = loader.get_module(get_angreal_task_path(file))
    members = dict(inspect.getmembers(module))
    tasks = loader.load_tasks(members)

    return set([task.name for task in tasks] + DOIT_CMDS)


def get_task_docs(file=DEFAULT_FILE):
    """
    Get task documentation w/o execution
    :param file:
    :return:
    """

    module = loader.get_module(get_angreal_task_path(file))
    members = dict(inspect.getmembers(module))
    tasks = loader.load_tasks(members)

    return "\n".join(['    {}        {}'.format(task.name,task.doc) for task in tasks])


def get_angreal_task_path(file=DEFAULT_FILE):
    """
    Attempts to find the angreal_tasks file by traversing parent directories until it's found.

    :param file: location of your angreal_tasks file.
    :return: path
    """

    file = list(os.path.split(file))
    current_path = Path(os.getcwd())

    angreal_path = None

    paths_to_test = [os.getcwd()] + list(current_path.parents)

    for p in paths_to_test:
        test_path = os.path.join(p, *file)
        if os.path.isfile(test_path):
            angreal_path = test_path
            break

    if not angreal_path:
        raise FileNotFoundError("Unable to find angreal_task file {}.".format(os.path.join(*file)))

    return angreal_path
