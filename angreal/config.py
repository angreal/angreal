"""
This module reads current configurations so that angreal tasks like init and update can be performed.
"""

import configparser
import logging
import json
from angreal import global_config
from angreal import user_config




module_logger = logging.getLogger(__name__)


class AngrealConfig(object):
    def __init__(self, file=None,angreal_name=None):
        if file:
            self.process_config(file,is_global=True)
        else :
            self.valid_sections = {'structure', 'githost', 'template_variables', 'behaviour','user'}
    
            self.structure = {
                'angreal_name': angreal_name,
                'directories' : [],
                'files'       : [],
                'templates'   : []
            }
    
            self.behaviour = {
                'append'  : [],
                'override': ['token']
            }
    
            self.githost = {
                'hostname' : '',
                'api'      : '',
                'labels'   : [],
                'milestone': [],
            }
    
            self.user = {
                'token' : ''
                }
            self.template_variables = {}
            
        self.process_config(global_config,is_global=True)
        self.process_config(user_config)


    def process_config(self, file_path, is_global=False):
        """
        Adds to the AngrealConfiguration object given a file_path.
        :param file_path: path to an angreal config file
        :param is_global: whether or not to treat a config file as global
        :return:
        """
        module_logger.warning('processing {}'.format(file_path))
        def set_variable(d, v, s):
            if isinstance(d[v], list):
                if v in self.behaviour['append'] or is_global:
                    value = [v.strip() for v in config.get(s, v, fallback=None).split('\n')]
                    d[v] += value

            elif isinstance(d[v], str):
                if v in self.behaviour['override'] or is_global:
                    value = config.get(s, v, fallback=None)
                    d[v] = value


        config = configparser.ConfigParser()
        config.read(file_path)
        
        if is_global :
            config.set('structure', 'angreal_name', self.structure['angreal_name'])

        for section in config:
            if section in self.valid_sections:
                for variable in config[section]:
                    if section is 'template_variables':
                        if variable[:2] is 't_':
                            self.template_variables[variable] = config.get(section, variable)

                    elif variable in self.structure.keys():
                        set_variable(self.structure, variable, section)

                    elif variable in self.githost.keys():
                        set_variable(self.githost, variable, section)

                    elif variable in self.behaviour.keys() and is_global:  # behavior items only set in global context
                        set_variable(self.behaviour, variable, section)
                    
                    elif variable in self.user.keys() and not is_global:
                        set_variable(self.user,variable,section)

            else:
                if section is not "DEFAULT":
                    module_logger.warning('section "{}" is not a valid section name'.format(section))


    def dump(self, filepath=None):
        """
        Dumps
        :param filepath:
        :return:
        """
        if not filepath:

            return json.dumps({
                'structure' : self.structure,
                'githost'   : self.githost,
                'template_variables'  : self.template_variables
                },indent=4, separators=(',', ': '))
        else:
            angreal_config = configparser.ConfigParser()

            def add_to_angreal_config(dictionary, section):
                angreal_config.add_section(section)
                for k, v in dictionary.items():
                    if isinstance(v, list):
                        v_to_l = '\n'.join(v)
                        angreal_config.set(section, k, v_to_l)
                    else:
                        angreal_config.set(section, k, v)

            add_to_angreal_config(self.githost, 'githost')
            add_to_angreal_config(self.structure, 'structure')
            add_to_angreal_config(self.template_variables, 'template_variables')

            with open(filepath, 'w') as f:
                angreal_config.write(f)


if __name__ == '__main__':
    project = AngrealConfig(angreal_name='test')
    print(project.dump())
