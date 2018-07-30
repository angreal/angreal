"""
    angreal.cutter
    ~~~~~~~~~~~~~~

    Angreal's templating functionality, leaning heavily on cookie cutter.
"""

import json
import os
import shutil

from cookiecutter.main import cookiecutter


def initialize_cutter(template, **kwargs):
    """
    Just pass through to cookie cutter making sure to get the replay
    :param template:
    :return:
    """
    kwargs.pop('replay', None)

    project_path = cookiecutter(template, **kwargs)

    #strip trailing slashes
    if template.endswith('/'):
        template = template[:-1]

    template_name = os.path.split(template)[-1]
    if template_name.endswith('.git'):
        template_name = template_name[:-4]


    project_name = os.path.split(project_path)[-1]
    angreal_hidden = os.path.join(project_path, '.angreal')
    generated_replay = os.path.join(os.environ.get('HOME'), '.cookiecutter_replay', '{}.json'.format(template_name))

    assert os.path.isfile(generated_replay)


    os.makedirs(angreal_hidden, exist_ok=True)



    #get the cookiecutter data
    with open(generated_replay,'r') as f:
        cookiecutter_data = json.load(f)['cookiecutter']


    #save it to disk
    angreal_replay = os.path.join(angreal_hidden,'angreal-replay.json')
    with open(angreal_replay,'w') as f:
        json.dump(cookiecutter_data,f)


    return project_path


