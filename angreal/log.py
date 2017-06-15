"""
Shamelessly adapted from Camille Scott's work.
"""
import logging
import os
import sys
import textwrap


class LogFormatter(logging.Formatter):
    def __init__(self, width=90, padding=10):
        super(LogFormatter, self).__init__('')
        self.width = width
        self.padding = padding
    
    def do_wrap(self, msg, pad):
        wrapped = textwrap.wrap(msg, self.width - self.padding)
        return ['{0}{1}'.format(pad, ln) for ln in wrapped]
    
    def format(self, record):
        if record.levelno < 40:
            pad = ' ' * self.padding
            wrapped = self.do_wrap(record.msg, pad)
            res = '\n'.join(wrapped) + '\n'
        else:
            extra = '[{0}:{1}]'.format(record.name, record.levelname)
            res = record.msg + extra
        
        return res


class AngrealLogger(object):
    '''Set up logging for the dammit application. We insulate it
    in a class to let us choose to only activate it when the program itself
    is run, effectively keeping the tests and any use of the API from
    being noisy.
    '''
    
    def __init__(self):
        self.log_file = os.path.join(os.environ['HOME'],'angreal.log')

        
        self.config = {'format'  : '%(asctime)s %(name)s:%(funcName)s:%(lineno)d ' \
                                   '[%(levelname)s] \n%(message)s\n-----',
                       'datefmt' : '%m-%d %H:%M:%S',
                       'filename': self.log_file,
                       'filemode': 'a'}
        
        # By default, only log errors (to the console)
        self.logger = logging.getLogger(__name__)
        noop = logging.NullHandler()
        self.logger.addHandler(noop)
    
    def run(self):
        logging.basicConfig(level=logging.DEBUG, **self.config)
        
        self.console = logging.StreamHandler(sys.stderr)
        self.console.setLevel(logging.INFO)
        self.formatter = LogFormatter()
        self.console.setFormatter(self.formatter)
        logging.getLogger('').addHandler(self.console)
        logging.getLogger('').debug('*** the wheel turns ***')


