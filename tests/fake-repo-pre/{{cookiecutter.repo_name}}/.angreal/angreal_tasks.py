DOIT_CONFIG = {'verbose': 2}


def task_xxx1(p1):
    """task doc"""
    return {
        'actions': ['do nothing'],
        'params': [{'name': 'p1', 'default': '1', 'short': 'p'}],
    }


def task_yyy2():
    return {'actions': None}


def not_a_task():
    """
    Shouldn't show up in loaded tasks
    :return:
    """
    return None
