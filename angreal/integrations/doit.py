"""
    angreal.integrations.doit
    ~~~~~~~~~~~~~~~~~~~~~~~~~

    Angreal integrates with pydoit
"""
from collections import Callable
from functools import wraps

from doit.cmd_base import TaskLoader
from doit.doit_cmd import DoitMain
from doit.task import dict_to_task


def make_doit_task(f):
    """
    Function decorator takes a pydoit `Task` dictionary returning a pydoit `Task` object.

    :param task_dict_function: pydoit dictionary generator
    :return:
    """
    @wraps(f)
    def d2t(*args, **kwargs):
        r_dict = f(*args, **kwargs)

        # Tasks require names, if they're not set default to the function name
        if 'name' not in r_dict.keys():
            r_dict['name'] = f.__name__
        return dict_to_task(r_dict)

    return d2t


# noinspection PyDefaultArgument
def run_doit_tasks(tasks, args, config={'verbosity': 0}):
    """
    Runs a series of task objects.

    :param tasks:
    :param args:
    :param config:
    :return:
    """

    if not isinstance(tasks, list):
        tasks = [tasks]

    tasks = [task() if isinstance(task, Callable) else task for task in tasks]

    class Loader(TaskLoader):
        @staticmethod
        def load_tasks(cmd, opt_values, pos_args):
            return tasks, config

    return DoitMain(Loader()).run(args)


def doit_task(f):
    """
    Execute a single function as though it was a doit task

    :param f:
    :return:
    """

    @wraps(f)
    def run_task(*args, **kwargs):
        r_dict = f(*args, **kwargs)

        # Tasks require names, if they're not set default to the function name
        if 'name' not in r_dict.keys():
            r_dict['name'] = f.__name__

        run_doit_tasks([dict_to_task(r_dict)],
                       ['run'])

    return run_task
