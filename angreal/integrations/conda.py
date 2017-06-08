import os
from distutils.spawn import find_executable
import logging
import subprocess

module_logger = logging.getLogger(__name__)


class CondaException(Exception):
    """
    condaException
    """
    
    def __init__(self, message):
        super().__init__(message)


class Conda(object):
    """ Hyper light weight wrapper for conda"""
    
    def __init__(self, working_dir=os.getcwd()):
        """ Constructor for Conda"""
        
        self.conda_path = find_executable('conda')
        self.working_dir = os.path.abspath(working_dir)
        

        if not self.conda_path:
            module_logger.exception('conda not in path')
            raise OSError('conda not in path')
    
    def __call__(self, command, *args, **kwargs):
        """
        :param command: the command to run
        :param args: arguments to add to command
        :param kwargs: keyword arguments to pass to conda
        :return:
        """
        
        # unpack a command (conda init --this=that -t=7 repo)
        system_call = (
            ('conda', command) +
            args
                +
            tuple(('--{0}={1}'.format(k, v) if len(k) > 1
                   else '-{0} {1}'.format(k, v))
                  for k, v in kwargs.items())
        )
        
        module_logger.debug('{} recieved.'.format(system_call))
        process_return = subprocess.run(system_call, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                        cwd=self.working_dir)
        module_logger.debug('{}'.format(process_return))
        
        if process_return.returncode != 0:
            message = 'conda non-zero exit status ({2}): {0} {1}'.format(process_return.args, process_return.stderr,
                                                                       process_return.returncode)
            module_logger.exception(message)
            raise CondaException(message)
        return
    
    def __getattr__(self, name, *args, **kwargs):
        """
        run a command as a method
        """
        return lambda *args, **kwargs: self(name, *args, **kwargs)

