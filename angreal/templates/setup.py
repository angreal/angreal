import sys
import os
import setuptools

VERSION = open('{{ project_name }}/VERSION').readline().strip()


py_version_tag = '-%s.%s'.format(sys.version_info[:2])


if not sys.version_info >= (3,0):
    print('Python 3 is required',file=sys.stderr)
    exit(1)

with open('requirements.txt' ,'r') as f:
    requirements = f.read().splitlines()


setuptools.setup(
    name='{{ project_name }}',
    description='',
    long_description='''
    ''',
    url='{{ project_url }}',
    author='{{ t_author }}',
    author_email='{{ t_email }}',
    license='',
    packages=['{{ project_name }}'],
    install_requires = requirements,
    zip_safe=False,
    version=VERSION,
    entry_points={
        'console_scripts': [
            '{{ project_name }} = {{ project_name }}.app:main'
            ]
    }
)
