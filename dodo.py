"""
doit file for repo maintenance
"""
import doit
from doit.tools import run_once
import os
import sys
import glob
from shutil import rmtree
import subprocess



def task_test_environment():
    """
    explicitly tests that we're in the correct environment
    :return:
    """
    try:
        conda_enviroment = os.path.basename(os.environ['CONDA_DEFAULT_ENV'])
    except KeyError:
        conda_enviroment = None
    
    if conda_enviroment == 'angreal':
        return{
            'actions' : None
        }
    else:
        msg ="""
You need to have a conda enviroment running via the following:
conda env create -f enviroment.yml
source activate angreal
"""
        print(msg, file=sys.stderr)
        


def task_update_enviroment():
    """
    Upadates the conda enviroment and stores it in the enviroment.yml file
    """
    return{
            'actions' :['conda env export -n angreal | grep -v \'#\' | grep -v \'prefix:\' > enviroment.yml'],
            'targets' :['enviroment.yml']
            }





def task_tests():
    """
    Runs unit tests via nose with coverage.
    """
    return {
        'actions' : ['nosetests -s --with-coverage --cover-package angreal --cover-html --cover-erase'],
        }


def task_sphinx():
    """
    Adds all files from the library to the sphinx api.
    """
    
    return {
        'actions' : ['sphinx-apidoc -fMeET -o docs/source/ angreal/'],
     }



def task_docs():
    """
    Cleans and generates docs.
    """
    return{
        'actions' : ['cd docs && make html']
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
