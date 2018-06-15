"""
    angreal.cutter
    ~~~~~~~~~~~~~~

    Angreal's templating functionality, leaning heavily on cookie cutter.
"""

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

    template_name = os.path.split(template)[-1]
    if template_name.endswith('.git'):
        template_name = template_name[:-4]


    project_name = os.path.split(project_path)[-1]
    angreal_hidden = os.path.join(project_path, '.angreal')
    generated_replay = os.path.join(os.environ.get('HOME'), '.cookiecutter_replay', '{}.json'.format(template_name))


    assert os.path.isfile(generated_replay)


    os.makedirs(angreal_hidden, exist_ok=True)

    shutil.move(generated_replay, os.path.join(angreal_hidden, '{}-replay.json'.format(project_name)))

    return project_path


