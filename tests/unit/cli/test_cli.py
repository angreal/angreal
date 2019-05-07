from angreal.cli import AngrealCLI
from tests import AngrealTest


class TestCLI(AngrealTest):

    def test_list_commands(self):
        """
        test list commands
        """
        test_cli = AngrealCLI({})
        tasks = test_cli.list_commands({})
        self.assertListEqual(tasks, ['test_1','test_2'])

    def test_get_commands(self):
        """
        test get commands
        """

        test_cli = AngrealCLI({})

        command = test_cli.get_command({},'test_1')
        print(command)


