from angreal.task_helpers import *
import pytest


def test_check_environment():
    """
    test check environment
    """
    os.environ.setdefault('TEST','test')
    check_environment('TEST')
    check_environment('test')
    os.environ.pop('TEST')
    return


def test_bad_environment():
    """
    test check environment bad
    """
    with pytest.raises(EnvironmentError):
        check_environment('test')


def test_file_exists():
    """
    test check files
    """
    check_files(__file__)


def test_file_no_exist():
    """
    test check files failes
    """
    with pytest.raises(FileNotFoundError):
        check_files('REALLYSHOULDNTEXIST')


def test_copy():
    """
    test copy file
    """
    open('test','w')
    copy_files('test2','test')
    assert os.path.isfile('test')
    assert os.path.isfile('test2')
    os.unlink('test')
    os.unlink('test2')

def test_clean_files():
    """
    test clean files
    """
    open('test','w')
    assert os.path.isfile('test')
    clean_files('test')
    assert not os.path.isfile('test')

def test_project_source_files():
    """
    test project source files
    """
    for f in  get_project_source_files('angreal'):
        assert os.path.isfile(f)
