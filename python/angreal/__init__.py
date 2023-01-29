"""
Angreal's Python API

"""
from .angreal import *
from .angreal import get_root as _get_root


__doc__ = angreal.__doc__
if hasattr(angreal, "__all__"):
    __all__ = angreal.__all__



def command(name:str=None, about: str="", long_about:str="", **attrs) ->None :   
    """decorator that identifies a function as an angreal task

    Args:
        name (str, optional): the name to be used to invoke a task. Defaults to the function name.  
        about (str, optional): A short description of what the task does. Defaults to "".
        long_about (str, optional): A longer description of what the task does. Defaults to the docstring on the decorated function.

    """
    _wrapped = None

    if callable(name):
        _wrapped = name
        name = _wrapped.__name__.lower().replace("_", "-")


    def decorator(f, long_about=None):

        if not hasattr(f, "__arguments"):
            f.__arguments = []
                        
        long_about = long_about or f.__doc__                        
        angreal.Command(name=name, about=about, long_about=f.__doc__, func=f)

        for arg in f.__arguments :
            Arg(**{**arg, **dict(command_name=name)})
        
        return f

    if _wrapped is not None:
        return decorator(_wrapped,long_about=long_about)

    return decorator


def argument(name,    
        python_type: str = "str",
        takes_value: bool = True,
        default_value: str = None,
        require_equals: bool = None,
        multiple_values: bool = None,
        number_of_values: int = None,
        max_values: int = None,
        min_values: int = None,
        short: str = None,
        long: str = None,
        long_help: str = None,
        help: str = None,
        required: bool = None, 
        **kwargs):
    """decorator that adds an argument to an angreal task

    Args:
        name (str): the argument name, must match a corresponding function argument
        python_type (str, optional): the python type to pass the value as. Must be one of ("str","int","float") . Defaults to "str".
        takes_value (bool, optional): doest the argument consume a trailing value. Defaults to True.
        default_value (str, optional): The default value to apply if none is provided. Defaults to None.
        require_equals (bool, optional): The consumed value requires an equal sign (i.e.`--arg=value`). Defaults to None.
        multiple_values (bool, optional): The argument takes multiple values. Defaults to None.
        number_of_values (int, optional): The argument takes a specific number of values. Defaults to None.
        max_values (int, optional): The argument takes at most X values. Defaults to None.
        min_values (int, optional): The argument takes at least X values. Defaults to None.
        short (str, optional): The short (single character) flag for the argument (i.e. `-i in the cli` would be `i`). Defaults to None.
        long (str, optional): The short (single word) flag for the argument (i.e. `--information` in the clie would be `information`). Defaults to None.
        long_help (str, optional): The help message to display with "long help" is requested with `--help`. Defaults to None.
        help (str, optional): The help message to display when help is requested via `-h`. Defaults to None.
        required (bool, optional): Whether the argument is required or not. Defaults to None.
    """
    def decorator(f):
        keyword_args = kwargs.copy()

        if not hasattr(f, "__arguments"):
            f.__arguments = []

        f.__arguments.append({**dict(name=name), **keyword_args})

        return f

    return decorator


def get_root():
    return _get_root()

def main():
    angreal.main()
