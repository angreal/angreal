"""
doit file for repo maintenance
"""
import doit
from doit.tools import run_once
import glob
import os




def task_update_enviroment():
    """
    Upadates the conda enviroment and stores it in the enviroment.yml file
    """
    try:
        conda_enviroment = os.environ['CONDA_DEFAULT_ENV']
    except KeyError:
        conda_enviroment = None
    
    if conda_enviroment == 'angreal':
        return{
            'actions' :['conda env export | grep -v \'#\' | grep -v \'prefix:\' > enviroment.yml'],
            'targets' :['enviroment.yml']
            }

    return {
        'actions' : None
            }


def task_tests():
    """
    Runs unit tests via nose with coverage.
    """
    pass


def task_sphinx_api():
    """
    Adds all files from the library to the sphinx api.
    """
    
    return {
        'actions' : ['sphinx-apidoc -fM -o docs/ angreal/'],
     }



def task_docs():
    """
    Cleans and generates docs.
    """
    return{
        'actions' : ['cd docs && make clean && make html']
        }
    pass
