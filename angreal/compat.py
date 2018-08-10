"""

    angreal.compat
    ~~~~~~~~~~~~~~

    Utilities for checking compatibility between an angreal template and the executor

"""
import os
import semver
import angreal
from angreal.utils import get_angreal_path



def is_compat(template_semver):
    """
    Compare a sematic version against `angeral.__version__`
    :param template_semver:
    :return:
    """

    return semver.match(angreal.__version__, template_semver)


def get_template_version():
    """
    Get the required version from the template.

    :return: the semantic version required to operate
    :rtype: str
    """
    version_file = os.path.join(get_angreal_path(), 'VERSION')
    if os.path.exists(  version_file ):
        return open(version_file).read().strip()
    return None
