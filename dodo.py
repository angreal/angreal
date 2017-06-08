"""
doit file for repo maintenance
"""
import doit
from doit.tools import run_once
import os
import glob
from shutil import rmtree
import subprocess




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
    return {
        'actions' : ['nosetests -s --with-coverage --cover-package angreal --cover-html --cover-erase'],
        }


def task_docs():
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




def task_cleaner():
    """
    cleans the repo
    :return:
    """

    def clean_coverage():
        if os.path.exists('.coverage'):
            os.unlink('.coverage')
        if os.path.isdir('cover'):
            rmtree('cover')

    def clean_doit():
        for i in glob.glob('.doit.db.*'):
            if os.path.exists(i):
                os.unlink(i)

    def clean_docs():
        os.chdir('docs')
        subprocess.call(['make', 'clean'])
        os.chdir('..')
        
    return{
        'actions' : None,
        'clean'   : [clean_coverage, clean_doit, clean_docs]
    }
