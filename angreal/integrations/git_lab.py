import gitlab
import logging
import getpass
import colour


module_logger = logging.getLogger(__name__)


class GitLabHost(object):
    """  """
    
    def __init__(self,api_url, token=None):
        """ Constructor for GitLabHost"""

        self.api_url = api_url
        self.__token = None
        self.can_connect = False

        if not token:
            self.__token = self.get_token()
        else :
            self.__token = token

        self.gl = gitlab.Gitlab(self.api_url, self.__token)
        self.gl.auth()

        self.project = None


    def get_group_id(self,name_space):
        """
        searches for a group_id given a name
        
        :param name_space: 
        :return: 
        """
        group_id = self.gl.groups.search(name_space)
        if len(group_id) > 1 :
            msg = 'returned multiple group_ids for name_space {}'.format(name_space)
            module_logger.error(msg)
            raise ValueError(msg)

        if len(group_id) == 0:
            msg = 'returned no group_ids for name_space {}'.format(name_space)
            module_logger.error(msg)
            raise ValueError(msg)

        return group_id[0].id

    def create_project(self,name,name_space=None):
        """
        creates a project on the gitlab host
        
        :param name:
        :param name_space: 
        """
        if name_space:
            name_space_id = self.get_group_id(name_space)
            self.project = self.gl.projects.create({
                                    'name' : name,
                                    'namespace_id' : name_space_id
                                    })

        else:
            self.project = self.gl.projects.create({
                                    'name': name
                                    })

    # noinspection PyUnreachableCode
    def delete_label(self,label_id=None,label_name=None):
        """
        Delete a label from a project based on label_id or label_name. 
        If both are provided, it will check that they're the same.
        
        .. todo:
        This doesn't appear to be working when manually testing.
        
        :param label_id:
        :param label_name:  
        :raises: ValueError
        """
        
        raise NotImplementedError
        
        if not self.project:
            msg = "Attempted to delete a label without a project object being set"
            module_logger.error(msg)
            raise ValueError(msg)
        
        t_label_id = None
        
        if label_name:
            t_label_id = self.project.labels.get(label_name)

        if label_id and label_name:
            if t_label_id != label_id:
                raise ValueError("label_id ({}) and label_name ({}) don't manage".format(label_id,label_name))

        if t_label_id :
            label_id = t_label_id

        self.project.labels.delete(label_id)

        logger.info("Deleted label from project")

        self.project.labels.create({'name':name , 'color':colour.Color(color).hex_1})

    def create_label(self,name,color):
        """
        creates a label for a project given a name and a color
        
        :param name:
        :param color: 
        :raises ValueError:
        """
        if not self.project:
            msg = "Attempted to create a label without a project object being set"
            module_logger.error(msg)
            raise ValueError(msg)

        color = colour.Color(color).hex_l

        module_logger.info("Added label {} -> {} to project".format(name,color))
        self.project.labels.create({'name':name , 'color':color})





    def create_milestone(self,name,description=None):
        """
        creates a milestone for a project
        :param name: The name of the milestone
        :raises ValueError: 
        """
        if not self.project:
            msg = "Attempted to create a milestone with out a project being set"
            module_logger.error(msg)
            raise ValueError(msg)
        if not description:
            description = ''
        self.project.milestones.create({'title': name,
                                        'description' : description
                                        })



    def get_token(self):

        if not self.__token:

            print("Looks like we need to get your personal token.")
            user_name = input("UserName:")
            password  = getpass.getpass()
            module_logger.info("Generating user token for {0} at {1}".format(user_name,self.api_url))

            gl = gitlab.Gitlab(self.api_url, email=user_name, password=password)
            gl.auth()
            self.__token = gl.private_token
            self.can_connect = True
            return self.__token

        return self.__token


        pass
