import sys
import setuptools
import os



VERSION = open(os.path.join('angreal', 'VERSION')).read().strip()
py_version_tag = '-%s.%s'.format(sys.version_info[:2])

if not sys.version_info >= (3, 0):
    print('Python 3 is required', file=sys.stderr)
    exit(1)



requirements = [
    'cookiecutter>=1.6.0,<2.0.0',
    'click>6.0',
    'python-gitlab>=1.5.1,<2.0.0',
    'semver>=2.8.1',
    'virtualenv>=16.0.0',
    'PyGithub>=1.43.7,<2.0.0',
    'docker>=3.4.1',
    'future',
]

dev_requirements = [
    'doit>=0.31.1',
    'pytest-cov==2.8.1',
    'sphinx_rtd_theme==0.4.0',
    'sphinx==1.7.5',
    'bs4==0.0.1',
    'lxml==4.2.3',
    'polling==0.3.0',
    'pytest==5.2.1',
    'twine==3.1.1',
]


setuptools.setup(
    name='angreal',
    description='making data science projects portable and consistent',
    long_description='''''',
    url='https://gitlab.com/dylanbstorey/angreal',
    author='dylanbstorey',
    author_email='dylan.storey@gmail.com',
    license='GPLv3',
    packages=setuptools.find_packages(exclude=['tests*']),
    install_requires=requirements,
    zip_safe=False,
    version=VERSION,
    entry_points={
        'console_scripts': [
            'angreal = angreal.cli:angreal_cmd'
        ]
    },
    python_requires='>=3',
    include_package_data=True,
    tests_require=['nose'],
    test_suite='nose.collector',
    extras_require={
        'dev': dev_requirements
    }
)
