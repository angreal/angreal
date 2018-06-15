import doit
import os
import shutil
import glob


def task_tests():
    """
    Running nosetests
    :return:
    """
    DEPENDENCIES = [x for x in glob.glob('angreal/*.py', recursive=True) if x[:-3] != 'pyc']
    DEPENDENCIES += list([x for x in glob.glob('test/*.py', recursive=True) if x[:-3] != 'pyc'])
    return {
        'actions': ['nosetests --with-coverage --cover-package=angreal'],
        # 'file_dep': DEPENDENCIES,
        'targets': ['.coverage'],
    }


def task_cleanup():
    """
    clean up things that should never be checked in
    :return:
    """

    def clean():
        rm_dirs = ['.mypy_cache', 'cover','angreal.egg-info']
        rm_files = ['.coverage', 'test_broker.sqlite3', 'data_logs.sqlite3', '.doit.db.db']

        [rm_dirs.append(f) for f in glob.glob('**', recursive=True) if os.path.basename(f) == '__pycache__']
        [rm_files.append(f) for f in glob.glob('docs/source/*.rst', recursive=True) if package in f]

        rm_dirs = [d for d in rm_dirs if os.path.isdir(d)]
        rm_files = [f for f in rm_files if os.path.exists(f)]

        [os.unlink(f) for f in rm_files] + [shutil.rmtree(d) for d in rm_dirs]

    return {
        'actions': [clean]
    }

def task_coverage():
    """
    Running nosetests with coverage
    :return:
    """

    return {
        'actions': ['nosetests --logging-level=FATAL --with-coverage --cover-package angreal --cover-html --cover-erase ']
    }



def task_docs():
    """
    Build our documentation
    :return:
    """
    return{
        'actions' : ['sphinx-apidoc -fMTeE -o docs/source angreal/ && cd docs && make clean && make html'] ,
        }


def task_doc_coverage():
    """
    Find out what documentation we need to get finished
    :return:
    """
    os.environ['SPHINX_APIDOC_OPTIONS']='members'
    return {
        'actions' : ['sphinx-apidoc -fMTeE -o docs/source angreal/ && cd docs && make coverage && cat build/coverage/python.txt >&2'] ,
}
