"""
angreal command to initialize and template an angreal
"""
import logging
import os
import configparser

from ast import literal_eval


from angreal import global_config as global_config_path
from angreal.integrations import Conda,CondaException
from angreal.integrations import Git,GitException
from angreal.integrations import GitLabHost

import angreal.integrations.file_system as file_system



module_logger = logging.getLogger(__name__)



valid_sections = {'structure', 'githost', 'template_variables', 'behaviour'}

structure = {
    'project_name': '',
    'directories' : [],
    'files'       : [],
    'templates'   : []
}

behaviour = {
    'append'  : [],
    'override': []
}

githost = {
    'hostname' : '',
    'api'      : '',
    'tags'     : [],
    'milestones': [],
}

template_variables = {}



def init(args):
    try:

        if os.path.isfile('.angreal'):
            module_logger.error('This angreal has already been formed!')
            exit(0)

        project_name = os.path.basename(os.getcwd())
        angreal_build_config = configparser.ConfigParser()
        angreal_build_config.read(global_config_path)
        angreal_build_config.set('DEFAULT','project_name' ,project_name)
        template_variables['project_name'] = project_name
        
        def set_variable(d, v, s):
            if isinstance(d[v], list):
                # if v in behaviour['append']:
                value = [v.strip() for v in angreal_build_config.get(s, v, fallback=None).split('\n')]
                d[v] += value
    
            elif isinstance(d[v], str):
                # if v in behaviour['override']:
                value = angreal_build_config.get(s, v, fallback=None)
                d[v] = value
                    
        for section in angreal_build_config:
            if section in valid_sections:
                for variable in angreal_build_config[section]:
                    if section is 'template_variables':
                        if variable[:2] is 't_':
                            template_variables[variable] = angreal_build_config.get(section, variable)
                
                    elif variable in structure.keys():
                        set_variable(structure, variable, section)
            
                    elif variable in githost.keys():
                        set_variable(githost, variable, section)
            
                    elif variable in behaviour.keys():
                        set_variable(behaviour, variable, section)
    
            else:
                if section is not "DEFAULT":
                    module_logger.warning('section "{}" is not a valid section name'.format(section))
                
        

        #make directory structure
        for d in structure['directories']:
            file_system.make_dir(d)
        
        #touches
        for f in structure['files']:
            file_system.touch(f)
        
        #templates
        for t in structure['templates']:
            template_info = literal_eval(t)
            if not isinstance(template_info,tuple):
                msg = '{} is not a tuple, check your config files'.format(t)
                module_logger.error(msg)
                raise TypeError(msg)
            file_system.template_to_project(template_info[1],template_info[0],**template_variables)
            
        #git init and git add
        git = Git()
        git.init()
        git.add('.')
        git('commit','-am','initial project setup')
        
        gh = GitLabHost(api_url=githost['api'])
        gh.create_project(project_name)
        
        for tag in githost['tags']:
            tag_info = literal_eval(tag)
            gh.create_label(tag_info[0],tag_info[1])

            
        for m in githost['milestones']:
            gh.create_milestone(m)
            

        
        git('remote','add','origin',gh.project.http_url_to_repo)
        git.push('origin','master')
        
        
    except Exception as e:
        module_logger.info('An exception was thrown, cleaning up')
        raise

    else :
        # file_system.touch('.angreal')
        module_logger.info('Angreal {} successfully created!')








"""




        if file_path == common.get_global_config_path():
            is_global = True

        config = configparser.ConfigParser()
        config.read(file_path)
        config.set('structure', 'angreal_name', self.angreal_name)

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

            else:
                if section is not "DEFAULT":
                    module_logger.warning('section "{}" is not a valid section name'.format(section))
                    
                    
    def dump(self, filepath=None):

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

            with open('.angreal', 'w') as f:
                angreal_config.write(f)


if __name__ == '__main__':
    project = AngrealConfiguration('this')
    project.process_config(common.get_global_config_path())

    print(project.dump_current_config())

"""
