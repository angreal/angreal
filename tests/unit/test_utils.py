import os

from angreal.utils import get_angreal_path
from angreal import warn,win,error

import pytest


def test_get_angreal_task_path():
    """
    test that we can find the path to the projects angreal directory
    """
    original_dir = os.getcwd()
    os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
    path = get_angreal_path(dir='.angreal')
    assert os.path.exists(path)
    os.chdir(original_dir)


def test_from_not_root():
    """
    test that we recurse up correctly
    """
    original_dir = os.getcwd()
    os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo','fake-project'))
    path = get_angreal_path(dir='.angreal')
    assert os.path.exists(path)
    os.chdir(original_dir)



def test_get_angreal_task_path_bad():
    """
    bad file path raises FileNotFoundError
    """
    with pytest.raises(FileNotFoundError):
        get_angreal_path(dir='.noangreal')

def test_warn():
    """
    Test a warning
    """

    warn("THIS IS A WARNING")

def test_error():
    """
    Test an error
    """

    error("THIS IS AN ERROR")

def test_win():
    """
    Test a win
    """

    win("WINNING")