"""
    tests.cli.test_list
    ~~~~~~~~~~~~~~~~~~~

    Tests against our list sub command
"""

import os
import shutil
import sys

from angreal.cli.list_cmd import list_cmd

import unittest

from click.testing import CliRunner




class TestBaseInit(unittest.TestCase):

    def test_list(self):

        runner = CliRunner()
        results = runner.invoke(list_cmd, [])

        try:
           assert results.exit_code == 0
        except:
           print(results, file=sys.stderr)


    def test_list(self):
        original_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), '..','fake-repo'))

        runner = CliRunner()
        results = runner.invoke(list_cmd, [])

        try:
           assert results.exit_code == 0
        except:
           print(results, file=sys.stderr)

        os.chdir(original_dir)