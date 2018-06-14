import  unittest
import os



class AngrealTest(unittest.TestCase):


    def setUp(self):
        self.start_dir = os.getcwd()
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))

    def tearDown(self):
        os.chdir(self.start_dir)

