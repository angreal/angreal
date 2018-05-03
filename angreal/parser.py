import argparse
from angreal.utils import get_task_docs

def get_init_parser():
    """
    Angreal only has one command when it's being called outside of a templated project.
    :return:
    """
    parser = argparse.ArgumentParser(
        description="Initialize an angreal project from a template."
    )
    parser.add_argument('template', type=str, nargs=1, help='An angreal template to initialize from')
    return parser


def get_task_parser():
    """
    When called within a templated project, we want to make sure that the user is aware of the tasks that are available
    :return:
    """
    parser = argparse.ArgumentParser(
    usage='''angreal <task> [ ]

Specific tasks:
{}


General tasks:
    auto            automatically execute tasks when a dependency changes
    clean           clean action / remove targets
    forget          clear successful run status from internal DB
    ignore          ignore task (skip) on subsequent runs
    info            show info about a task
    list            list tasks from angreal tasks file


For more specific information about a command use:
    angreal help <command>
    '''.format(get_task_docs()),
    )
    parser.add_argument('task', help='a task to execute')
    return parser