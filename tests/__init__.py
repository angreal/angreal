import  unittest
import os
import functools


def return_to_cwd(f,*args,**kwargs):
    """
    ensure that we return to cwd after a test
    """
    @functools.wraps(f)
    def wrapper(*args,**kwargs):
        current_dir = os.getcwd()
        f(*args, **kwargs)
        os.chdir(current_dir)
    return wrapper




class AngrealTest(unittest.TestCase):


    def setUp(self):
        self.start_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))

    def tearDown(self):
        os.chdir(self.start_dir)

