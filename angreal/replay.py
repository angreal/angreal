"""
    angreal.replay
    ~~~~~~~~~~~~~~

    Class to work with the replay file.
"""

import fnmatch
import json
import os



from angreal.utils import get_angreal_path

class Replay(dict):
    """
    Replays are a subclassed dictionary that are meant to be used to track/modify project specific attributes.
    """


    def __init__(self,file=None):
        """
        Initialize the Replay object, if no file is provided angreal will attempt to find one in parent directories.

        :param file: the replay to load (defaults to looking in the .angreal directory)
        :type file: string
        """

        if not file: #Default, go try and find it
            file = []
            directory = get_angreal_path()
            for f in os.listdir(directory):
                if fnmatch.fnmatch(f,'angreal-replay.json'):
                    file.append(f)

            if len(file) > 1 :
                raise ValueError('Found multiple files matching the replay pattern.')

            file = os.path.join(directory,file[0])


        else:
            if not os.path.isfile(file):
                raise FileNotFoundError()


        self.file = file

        with open (self.file,'r') as f:
            here = json.load(f)

        super(Replay, self).__init__(**here)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.save()
        return


    def save(self):
        """
        save the current replay
        """
        with open(self.file,'w') as f:
            json.dump(self, f)