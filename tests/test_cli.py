import os

from angreal import AngrealCLI
from tests import AngrealTest


class TestCLI(AngrealTest):

    def test_list_commands(self):
        """
        Command names are properly parsed from file name.
        :return:
        """
        test_cli = AngrealCLI({})
        print(os.getcwd())
        tasks = test_cli.list_commands({})
        assert tasks == ['test_1', 'test_2']

    def test_get_commands(self):
        test_cli = AngrealCLI({})

        command = test_cli.get_command({},'test_1')
        print(command)


