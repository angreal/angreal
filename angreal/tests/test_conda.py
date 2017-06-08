from unittest import TestCase
from angreal import Conda
from angreal import CondaException
from shutil import rmtree
import tempfile
import os

tempdir = os.path.abspath(os.path.dirname(__file__))
tempenviroment = os.path.join(tempdir, 'environment.yml')



class TestConda(TestCase):
    @classmethod
    def setUpClass(cls):
        pass

    @classmethod
    def tearDownClass(cls):
        conda = Conda()
        conda.remove('-y', '-n' ,'condatest' ,'--all')
        os.unlink(tempenviroment)
        pass


    def test_conda_1(self):
        conda = Conda()



    def test_conda_create(self):
        conda = Conda(working_dir=tempdir)
        conda.create('-y','-n','condatest','python=3.5')

        try:
            conda.create('-y', '-n', 'condatest', 'python=3.5')
        except CondaException:
            pass

    def test_conda_export(self):
        conda = Conda(working_dir=tempdir)
        conda.env('export','-n' , 'condatest' ,'-f' ,tempenviroment)
        assert os.path.exists(tempenviroment)


