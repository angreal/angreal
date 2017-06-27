import doit
import glob
import os
all_files = glob.glob('{{ project_name }}/', recursive=True)


def task_update_conda():
    """
    
    :return: 
    """
    if os.environ['CONDA_DEFAULT_ENV'] is '{{ project_name }}':
        return {
            'actions': ['conda env export -n {{ project_name }} | grep -v "^prefix: " > environment.yml'],
            'targets': ['environment.yml'],
        }



def task_update_tests():
    """
    always run code coverage
    :return:
    """

    return {
        'actions': ['nosetests -s --with-coverage --cover-package {{ project_name }} --cover-html --cover-erase'],
        'targets': ['cover/index.html'],
    }

def task_api_doc():
    """
    always run sphinx-api
    :return: 
    """
    return {
        'actions' : ['sphinx-apidoc -fMeE -o docs/ {{ project_name }}/']
    }

def task_generate_documentation():
    """
    always re-make documentation
    :return: 
    """
    return{
        'actions' : ['cd docs && make html']
        }
