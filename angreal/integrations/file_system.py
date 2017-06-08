import os
import glob
import shutil
import logging

from jinja2 import Environment, FileSystemLoader
from jinja2.exceptions import TemplateNotFound

from angreal import static_files
from angreal import dynamic_files
from angreal import global_config

module_logger = logging.getLogger(__name__)



def copy_to_angreal(file,dst):
    """
    copy a file from static to destination
    :param file: The name of the static file
    :param dst:
    :raises FileNotFoundError:
    """
    src = os.path.abspath(os.path.join(static_files,file))
    dst = os.path.abspath(dst)
    
    if not os.path.isfile(src):
        msg = "file {} doesn't appear to have been registered".format(src)
        module_logger.error(msg)
        raise FileNotFoundError(msg)
        
    shutil.copy(src, dst)
    
    
def template_to_angreal(file, dst, **kwargs):
    """
    renders a template to a destination
    :param file: the template to render
    :param dst:  where it should wind up
    :param kwargs: template variables
    :return:
    """
    dst = os.path.abspath(dst)
    
    env = Environment(loader=FileSystemLoader(dynamic_files))
    
    try:
        template = env.get_template(file)
    except TemplateNotFound:
        module_logger.error("file {} doesn't appear to have been registered".format(file))
        raise
        
    with open(dst, 'w') as f:
        f.write(template.render(template_dict))

def register(src, dst):
    """
    Register a file INTO angreal
    :param src: path to file
    :param dst: destination
    :raises FileNotFoundError:
    :raises EnviromentError:
    """
    
    file_base = os.path.splitext(os.path.basename(src))[0]
    src = os.path.abspath(src)
    dst = os.path.join(dst,file_base)
    
    if not os.path.isfile(file):
        msg = "file {} doesn't appear to exist".format(src)
        module_logger.error(msg)
        raise FileNotFoundError(msg)
    
    if os.path.exists(dst):
        msg = 'file {} has already been registered'.format(dst)
        module_logger.error(msg)
        raise EnvironmentError(msg)
    
    shutil.copy(src,dst)


def register_file(file):
    """
    copy a static file to angreal's static_files dir
    :param file:
    :return:
    """
    register(file, static_files)

def register_template(file):
    """
    copy a template to angreal's dynamic_files dir
    :param file:
    :return:
    """
    register(file, dynamic_files)


def touch(file):
    """
    creates a file if it doesn't already exist. Doesn't actually update utime.
    :param file:
    :return:
    """
    open(file,'a').close()
    

def mdkir(dir):
    """
    makes a directory
    :param dir:
    :return:
    """
    os.makedirs(os.path.abspath(dir),exist_ok=True)
    pass


def set_read_on_global_files():
    """
    sets permissions to a+r on static and dynamic files
    :return:
    """
    [os.chmod(i,stat.S_IROTH) for i in glob.glob(os.path.join(static_files),'*')]
    [os.chmod(i,stat.S_IROTH) for i in glob.glob(os.path.join(dynamic_files),'*')]
    os.chmod(global_config,stats.S_IROTH)



