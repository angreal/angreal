import sys
import os
import setuptools

VERSION = open('angreal/VERSION').readline().strip()


py_version_tag = '-%s.%s'.format(sys.version_info[:2])


if not sys.version_info >= (3,0):
    print('Python 3 is required',file=sys.stderr)
    exit(1)

with open('requirements.txt' ,'r') as f:
    requirements = f.read().splitlines()


setuptools.setup(
    name='angreal',
    description='making data science models portable and consistent',
    long_description='''A package and script for creating data science packages formatted
    as a pip-like repository. Handles setting up and registering the
    project with git and web based git services like github and gitlab
    ''',
    url='http://gihub.com/dylanbstorey/angreal',
    author='dylanbstorey',
    author_email='dylan.storey@gmail.com',
    license='GPLv3',
    packages=['angreal'],
    install_requires = requirements,
    zip_safe=False,
    version=VERSION,
    entry_points={
        'console_scripts': [
            'angreal = angreal.app:main'
            ]
    }
)
