"""
angreal command for updating a current angreal

What does this mean ?

Probably:
- Update the enviroment with the current angreal.yml
- Update the angreal.yml with the current enviroment
- Check that the project exists on the webhost, if it doesnt create it.
"""

import logging


module_logger = logging.getLogger(__name__)

def update(args):
    print(args)
    print('UPDATING THIS ANGREAL')
    pass
