"""
    angreal.cutter
    ~~~~~~~~~~~~~~

    Angreal's templating functionality, leaning heavily on cookie cutter.
"""

import json
import os
import subprocess
import sys

from cookiecutter.main import cookiecutter


def initialize_cutter(template, **kwargs):
    """
    Just pass through to cookie cutter making sure to get the replay

    :param template:
    :return:
    """
    kwargs.pop('replay', None)


    template_path = None

    if os.path.isdir(template):
        template_path = template

        # strip trailing slashes
        if template.endswith('/'):
            template_path = template_path[:-1]

        template_name = os.path.split(template_path)[-1]
        if template_name.endswith('.git'):
            template_name = template_name[:-4]

    else:
        template = template.replace('-', '_') #All pypi based angreals use underscores and not dashes.
        rc = subprocess.call([sys.executable,'-m','pip','install','angreal_{}'.format(template)])
        if rc != 0 :
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
        cookiecutter_data = json.load(f)['cookiecutter']


    #save it to disk
    angreal_replay = os.path.join(angreal_hidden,'angreal-replay.json')
    with open(angreal_replay,'w') as f:
        json.dump(cookiecutter_data,f)


    return project_path


