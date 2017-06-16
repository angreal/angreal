"""
angreal subcommands for registering files
"""

import logging
import os
module_logger = logging.getLogger(__name__)
from angreal.integrations import file_system




def register(args):
    for i in args.file:
        module_logger.info('Registering {} to angreal'.format(i))
        try:
            file_system.register(i)
        except:
            pass
