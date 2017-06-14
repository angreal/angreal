import nose
import os
import angreal

where_am_i = os.path.abspath(os.path.dirname(__file__))


def test_folders_exist():
    assert os.path.isdir(os.path.join(where_am_i,'..','static_files'))
    assert os.path.isdir(os.path.join(where_am_i, '..','dynamic_files'))

def test_path_values():
    assert angreal.angreal_location == os.path.abspath(os.path.join(where_am_i,'..'))
    assert angreal.dynamic_files == os.path.abspath(os.path.join(where_am_i, '..','dynamic_files'))
    assert angreal.static_files == os.path.abspath(os.path.join(where_am_i, '..','static_files'))
