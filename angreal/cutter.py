"""
    angreal.cutter
    ~~~~~~~~~~~~~~

    Angreal's templating functionality, leaning heavily on cookie cutter.
"""

import json
import os
import subprocess
import sys
from angreal.integrations.cookiecutter import cookiecutter



def initialize_cutter(template, **kwargs):
    """
    Just pass through to cookie cutter making sure to get the replay

    :param template:
    :return:
    """
    kwargs.pop('replay', None)

    if template.endswith('/'):
        template = template[:-1]

    template_path = None
    template_name = None

    if os.path.isdir(template):  # the template is a directory
        template_path = template
        template_name = os.path.split(template)[-1]

    elif template.endswith('.git'):  # the template appears to be a remote git repo
        template_name = os.path.split(template)[-1]
        template_name = template_name[:-4]
        template_path = template

    else:  # the template is in pypi
        template = template.replace('-', '_')  # All pypi based angreals use underscores and not dashes.
        rc = subprocess.call([sys.executable, '-m', 'pip', 'install', '--upgrade', 'angreal_{}'.format(template)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

        if rc != 0:
            exit("failed to install angreal_{}".format(template))

        template_path = os.path.abspath(
                            os.path.join(
                                sys.prefix, 'angreal_{}'.format(template)
                            )
                )
        template_name = 'angreal_{}'.format(template)

    project_path = cookiecutter(template_path, **kwargs)



    angreal_hidden = os.path.join(project_path, '.angreal')
    generated_replay = os.path.join(os.environ.get('HOME'), '.cookiecutter_replay', '{}.json'.format(template_name))


    os.makedirs(angreal_hidden, exist_ok=True)



    #get the cookiecutter data
    with open(generated_replay,'r') as f:
        cookiecutter_data = json.load(f)['angreal']


    #save it to disk
    angreal_replay = os.path.join(angreal_hidden,'angreal-replay.json')
    with open(angreal_replay,'w') as f:
        json.dump(cookiecutter_data,f)


    return project_path


