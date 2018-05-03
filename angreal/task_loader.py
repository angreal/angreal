"""
    angreal.task_loader
    ~~~~~~~~~~~~~~~~~~~

    angreals task loader

"""

from doit import loader
from doit.cmd_base import TaskLoader

from angreal.utils import get_angreal_task_path

# cwd
opt_cwd = {
    'name': 'cwd_path',
    'short': 'd',
    'long': 'dir',
    'type': str,
    'default': None,
    'help': ("set path to be used as cwd directory (file paths on " +
             "angreal file are relative to angreal.tasks location).")
}

# seek dodo file on parent folders
opt_seek_file = {
    'name': 'seek_file',
    'short': 'k',
    'long': 'seek-file',
    'type': bool,
    'default': True,
    'env_var': 'DOIT_SEEK_FILE',
    'help': ("seek angreal.tasks file on parent folders " +
             "[default: %(default)s]")
}


class AngrealTaskLoader(TaskLoader):
    """default task-loader create tasks from a dodo.py file"""
    cmd_options = (opt_cwd, opt_seek_file)




    def load_tasks(self, cmd, params, args):
        dodo_module = loader.get_module(
            get_angreal_task_path(),
            params['cwd_path'],
            params['seek_file'])
        return self._load_from(cmd, dodo_module, self.cmd_names)
