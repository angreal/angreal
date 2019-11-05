"""
    angreal.integrations.cookiecutter.find
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Functions for finding Cookiecutter templates and other components.

    Used instead of cookiecutter.find so we can override some of the required syntaxes enforced by cookiecutter.

"""

import logging
import os

from cookiecutter.exceptions import NonTemplatedInputDirException

logger = logging.getLogger(__name__)


def find_template(repo_dir):# pragma: no cover
    """Determine which child directory of `repo_dir` is the project template.

    :param repo_dir: Local directory of newly cloned repo.
    :returns project_template: Relative path to project template.
    """
    logger.debug('Searching {} for the project template.'.format(repo_dir))

    repo_dir_contents = os.listdir(repo_dir)

    project_template = None
    for item in repo_dir_contents:
        if 'angreal' in item and '{{' in item and '}}' in item:
            project_template = item
            break

    if project_template:
        project_template = os.path.join(repo_dir, project_template)
        logger.debug(
            'The project template appears to be {}'.format(project_template)
        )
        return project_template
    else:
        raise NonTemplatedInputDirException