"""
angreal command for triggering a build process

What does 'build' mean , is it customizable,
¯\_(ツ)_/¯

"""
import logging


module_logger = logging.getLogger(__name__)

def build(args):
    print(args)
    print('building it')
