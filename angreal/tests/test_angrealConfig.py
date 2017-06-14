from unittest import TestCase
from angreal import AngrealConfig
import os
import json



test_confg = os.path.join(os.path.dirname(__file__), 'test.cfg')

class TestAngrealConfig(TestCase):
    
    def test_process_config(self):
        project = AngrealConfig(file=test_confg)
    
    
        print (project.behaviour)
        
        assert project.behaviour['append'] == ['tags']
        assert project.behaviour['override'] == ['token' ,'angreal_name']
        
        assert project.structure['angreal_name'] =='test'
        assert project.structure['directories'] == ['test']
    
    def test_dump(self):
        project = AngrealConfig(file=test_confg)
        dump = json.loads(project.dump())
        
