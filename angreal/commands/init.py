"""
angreal command to initialize and template an angreal
"""
import logging
import os
import configparser

from angreal import global_config as global_config_path
from angreal import user_config   as user_config

from angreal.integrations import Conda,CondaException
from angreal.integrations import Git,GitException

module_logger = logging.getLogger(__name__)

def init(args):
    try:
        if os.path.isfile('.angreal'):
            module_logger.error('This angreal has already been formed!')
            os.exit(0)

        project_name = os.path.basename(os.getcwd())
        angreal_build_config = configparser.ConfigParser()
        angreal_build_config['DEFAULT']['angreal_name'] = project_name

    except Exception as e:
        module_logger.info('An exception was thrown, cleaning up')
        raise

    else :
        module_logger.info('Angreal {} successfully created!')

