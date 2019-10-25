import doit
import fnmatch
import os
import shutil
import glob


DOIT_CONFIG = {
    'backend' : 'json',
    'dep_file': 'doit-db.json'
}


def task_functional_tests():
    """
    Running functional tests
    :return:
    """
    return {
        'actions': ['pytest -vvv --disable-pytest-warnings tests/functional'],
        'verbosity' : 2
    }


def task_tests():
    """
    Running nosetests
    :return:
    """
    return {
        'actions': ['pytest -svvvrxX --cov=angreal/ --cov-report html --cov-report=term --disable-pytest-warnings tests/unit'],
        'verbosity': 2
    }


def task_cleanup():
    """
    clean up things that should never be checked in
    :return:
    """

    def clean():
        rm_dirs = ['.mypy_cache', 'cover','angreal.egg-info']
        rm_files = ['.coverage', 'test_broker.sqlite3', 'data_logs.sqlite3', '.doit.db.db','doit-db.json']

        [rm_dirs.append(f) for f in glob.glob('**', recursive=True) if os.path.basename(f) == '__pycache__']
        [rm_files.append(f) for f in glob.glob('docs/source/*.rst', recursive=True) if 'angreal' in f]

        rm_dirs = [d for d in rm_dirs if os.path.isdir(d)]
        rm_files = [f for f in rm_files if os.path.exists(f)]

        [os.unlink(f) for f in rm_files] + [shutil.rmtree(d) for d in rm_dirs]

    return {
        'actions': [clean]
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


def task_loc():
    """
    Get current LOC in project
    :return:git
    """

    def walk(root='.', recurse=True, pattern='*'):
        """
            Generator for walking a directory tree.
            Starts at specified root folder, returning files
            that match our pattern. Optionally will also
            recurse through sub-folders.
        """
        for path, subdirs, files in os.walk(root):
            for name in files:
                if fnmatch.fnmatch(name, pattern):
                    yield os.path.join(path, name)
            if not recurse:
                break

    def loc(root='', recurse=True):
        """
            Counts lines of code in two ways:
                maximal size (source LOC) with blank lines and comments
                minimal size (logical LOC) stripping same

            Sums all Python files in the specified folder.
            By default recurses through subfolders.
        """
        count_mini, count_maxi = 0, 0
        for fspec in walk(root, recurse, '*.py'):
            skip = False
            for line in open(fspec).readlines():
                count_maxi += 1

                line = line.strip()
                if line:
                    if line.startswith('#'):
                        continue
                    if line.startswith('"""'):
                        skip = not skip
                        continue
                    if not skip:
                        count_mini += 1

        return "\'Lines of Code: {0} \nTotal Lines: {1}\'".format(count_mini, count_maxi)

    return {
        'actions' : ['echo %s >&2 ' % loc('.')],
}