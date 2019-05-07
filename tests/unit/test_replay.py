"""
    tests.test_replay
    ~~~~~~~~~~~~~~~~~

    angreal.replay testing suite


"""
import os
import sys
import unittest

from angreal.replay import Replay


test_file = os.path.join( os.path.dirname(__file__), 'fake-repo','.angreal','angreal-replay.json')

class TestReplay(unittest.TestCase):


    def test_init(self):
        replay = Replay(file = test_file)
        assert replay.file == test_file

    def test_save(self):
        replay = Replay(file=test_file)
        replay.save()

    def test_init_no_file(self):
        here = os.getcwd()
        os.chdir(os.path.join( os.path.dirname(__file__), 'fake-repo'))
        replay = Replay()
        assert replay.file == test_file
        os.chdir(here)

    def test_context(self):

        with Replay(file=test_file) as r :
            pass