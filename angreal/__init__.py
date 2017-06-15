import os

from angreal import log


angreal_location = os.path.abspath(os.path.dirname(__file__))

templates_dir = os.path.join(angreal_location,'templates')


global_config = os.path.join(angreal_location,'global.cfg')
user_config = os.path.join(os.path.expanduser('~'),'.angrealrc')




from angreal.integrations.git import Git
from angreal.integrations.git import GitException

from angreal.integrations.conda import Conda
from angreal.integrations.conda import CondaException

from angreal.integrations.git_lab import GitLabHost
import angreal.integrations.file_system



__version__ = open(os.path.join(angreal_location , 'VERSION'),'r').read().strip()
