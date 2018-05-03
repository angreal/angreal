import os
from doit.cmd_base import Command
from angreal.task_loader import AngrealTaskLoader


class TestAngrealTaskLoader(object):
    def test_load_tasks(self):
        """
        Test angreal task loader
        :return:
        """
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        cmd = Command()
        params = {
            'cwd_path': None,
            'seek_file': True,
        }
        loader = AngrealTaskLoader()
        task_list, config = loader.load_tasks(cmd, params, [])
        assert ['xxx1', 'yyy2'] == [t.name for t in task_list]
        assert {'verbose': 2} == config
        os.chdir(original_dir)

    def test_seek_tasks(self):
        """
        Test angreal task loader seeks
        :return:
        """
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        os.makedirs('test_temp', exist_ok=True)
        os.chdir('test_temp')
        cmd = Command()
        params = {
            'cwd_path': None,
            'seek_file': True,
        }
        loader = AngrealTaskLoader()
        task_list, config = loader.load_tasks(cmd, params, [])
        assert ['xxx1', 'yyy2'] == [t.name for t in task_list]
        assert {'verbose': 2} == config

        os.chdir('..')
        os.rmdir('test_temp')
        os.chdir(original_dir)
