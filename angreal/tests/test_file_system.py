from unittest import TestCase

from angreal.integrations import file_system
from angreal import templates_dir
import os





class TestFileSystem(TestCase):


    def test_01_touch(self):
        test_file = os.path.join(templates_dir,'touch_test')
        file_system.touch(test_file)
        assert os.path.isfile(test_file)
        os.unlink(test_file)


    def test_02_mkdir(self):
        test_dir = os.path.join(templates_dir,'mkdir_test')
        file_system.make_dir(test_dir)
        assert os.path.isdir(test_dir)
        os.rmdir(test_dir)


    def test_03_set_read(self):
        test_file = os.path.join(templates_dir,'read_test')

        open(test_file,'w').close()

        file_system.set_read_on_global_files()

        try:
            open(test_file,'w').close()
        except PermissionError as e:
            pass

        os.unlink(test_file)


    def test_04_dir_is_emty(self):
        test_dir = os.path.join(templates_dir,'empty_dir_test')
        file_system.make_dir(test_dir)

        assert file_system.dir_is_empty(test_dir)

        test_add_file = os.path.join(test_dir,'test')
        file_system.touch(test_add_file)
        assert not file_system.dir_is_empty(test_dir)

        os.unlink(test_add_file)
        os.rmdir(test_dir)


    def test_05_register(self):
        try:
            file_system.register('noexist')
        except FileNotFoundError :
            pass

        test_file = os.path.join('read_test')
        open(test_file,'w').close()
        assert os.path.exists(test_file)
        file_system.register(test_file)
        assert os.path.exists(os.path.join(templates_dir,'read_test'))

        try :
            file_system.register(test_file)
        except EnvironmentError :
            pass

        os.unlink(test_file)
        os.unlink(os.path.join(templates_dir,'read_test'))

    def test_06_template_to_project(self):

        try:
            file_system.template_to_project('noexist','.')
        except EnvironmentError:
            pass


        test_file = os.path.join('read_test')
        open(test_file, 'w').close()
        assert os.path.exists(test_file)
        file_system.register(test_file)
        assert os.path.exists(os.path.join(templates_dir, 'read_test'))

        file_system.template_to_project('read_test','..')
        assert os.path.exists(os.path.join('..','read_test'))

        os.unlink(os.path.join('..','read_test'))
        os.unlink(test_file)
        os.unlink(os.path.join(templates_dir,'read_test'))
