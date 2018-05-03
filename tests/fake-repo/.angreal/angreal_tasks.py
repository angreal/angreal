import sys
DOIT_CONFIG = {'verbose': 2}


def task_xxx1():
    """Hello world with paramaeters/pos http://pydoit.org/task_args.html
    """
    def print_hello_pos(pos=None):
        """
        Demo of how to use a positional argument
        :param pos:
        :return:
        """
        if not pos:
            pos = ['World']
        print('Hello {}!'.format(pos[0]),file=sys.stderr)

    def print_hello_param(param1):
        """
        Demo of how to use a parameter argument
        :param param1:
        :return:
        """
        print('Hello {}!'.format(param1),file=sys.stderr)
    return {
        'actions': [print_hello_pos, print_hello_param],
        'pos_arg': 'pos',
        'params' : [{
                    'name' : 'param1',
                    'short': 'p',
                    'default' : 'world'
                    }]
    }


def task_yyy2():
    """ task two doc"""
    def print_2():
        print("Task 2", file=sys.stderr)
    return {
        'actions': [print_2],
    }


def not_a_task():
    """
    Shouldn't show up in loaded tasks
    :return:
    """
    return None
