import sys
import os
import setuptools

VERSION = '0.0.1a'


py_version_tag = '-%s.%s'.format(sys.version_info[:2])


if not sys.version_info >= (3,0):
    print('Python 3 is required',file=sys.stderr)
    exit(1)


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
    install_requires = [],
    zip_safe=False,
    version=VERSION
)
