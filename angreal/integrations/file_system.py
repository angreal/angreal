import os
import glob
import shutil
import logging
import stat

from jinja2 import Environment, FileSystemLoader, meta
from jinja2.exceptions import TemplateNotFound

from angreal import templates_dir
from angreal import global_config

module_logger = logging.getLogger(__name__)




def template_to_project(file, dst, **kwargs):
    """
    renders a template to a destination. If the template doesn't have any templated variables amounts to a copy.
    
    :param file: the template to render
    :param dst:  where it should wind up
    :param kwargs: template variables
    :return dict: template variables used
    """
    dst = os.path.abspath(dst)
    
    env = Environment(loader=FileSystemLoader(templates_dir))

    if os.path.isdir(dst):
        dst = os.path.join(dst,file)

    try:
        template = env.get_template(file)
    except TemplateNotFound:
        module_logger.error("file {} doesn't appear to have been registered".format(file))
        raise EnvironmentError("file {} doesn't appear to have been registered".format(file))

    #What was I passed
    template_dict = {**kwargs}
    
    #What do I need
    ast = env.parse(file)
    variables = meta.find_undeclared_variables(ast)
    
    #Diff
    unset_variables = variables - set(template_dict.keys())
    
    #Update
    for unset in unset_variables:
        template_dict[unset] = input('value for {}'.format(unset))
        
    with open(dst, 'w') as f:
        f.write(template.render(template_dict))
        
    return template_dict

def register(src, dst=templates_dir):
    """
    Register a file INTO angreal
    
    :param src: path to file
    :param dst: destination
    :raises FileNotFoundError:
    :raises EnviromentError:
    """
    
    file_base = os.path.basename(src)
    src = os.path.abspath(src)
    dst = os.path.join(dst,file_base)
    
    if not os.path.isfile(src):
        msg = "file {} doesn't appear to exist".format(src)
        module_logger.error(msg)
        raise FileNotFoundError(msg)
    
    if os.path.exists(dst):
        msg = 'file {} has already been registered'.format(dst)
        module_logger.error(msg)
        raise EnvironmentError(msg)
    
    shutil.copy(src,dst)



def touch(file):
    """
    creates a file if it doesn't already exist. Doesn't actually update utime.
    
    :param file:
    :return:
    """
    open(file,'a+').close()
    return
    

def make_dir(dir):
    """
    makes a directory
    
    :param dir:
    :return:
    """
    os.makedirs(os.path.abspath(dir),exist_ok=True)
    pass


def set_read_on_global_files():
    """
    sets permissions to a+r on files.
    
    :return:
    """
    [os.chmod(i, stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH ) for i in glob.glob(os.path.join(templates_dir,'*'))]
    os.chmod(global_config,stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)

def dir_is_empty(dir):
    """
    tests if a directory is empty or not
    
    :param dir:
    :return:
    """
    dir = os.path.abspath(dir)
    if not os.listdir(dir):
        return True
    return False


