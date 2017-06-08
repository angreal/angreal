"""


"""
import os

angreal_location = os.path.abspath(os.path.dirname(__file__))
static_files = os.path.join(angreal_location, 'static_files')
dynamic_files = os.path.join(angreal_location, 'dynamic_files')
global_config = os.path.join(angreal_location,'global.cfg')


from angreal.integrations.git import Git
from angreal.integrations.git import GitException

from angreal.integrations.conda import Conda
from angreal.integrations.conda import CondaException

from angreal.integrations.git_lab import GitLabHost
import angreal.integrations.file_system
