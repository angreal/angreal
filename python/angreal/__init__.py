from .angreal import *
import functools

__doc__ = angreal.__doc__
if hasattr(angreal, "__all__"):
    __all__ = angreal.__all__







def command(**kwargs):
    """The command decorator, used to register a user defined function as a subcommand for angreal to execute.

    Args:
        name (str): The name of the command
        about (str, optional): A short description of the commands function. Defaults to None.
        long_about (str, optional): A longer description of the commands function. Defaults to None.
    """

    def decorator(f):
        if not hasattr(f, "__arguments"):
            f.__arguments = []
                
        angreal.Command(**kwargs, func=f)

        for arg in f.__arguments :
            angreal.Arg(**{**arg, **dict(command_name=kwargs.get('name',f.__name__.lower().replace("_", "-")))})

        @functools.wraps(f)
        def wrapper(*f_args,**f_kwargs):
            return f(*f_args,**f_kwargs)
        
        return wrapper
    return decorator



def argument(**kwargs):
    """Register an argument as part of a command

    Args:
        name (str): the argument name. Should match a wrapped functions argument
        takes_value (bool, optional):  does the argument consumer a trailing value. Defaults to True.
        default_value (bool, optional): the default value to apply if none is provided. Defaults to None.
        requires_equals (bool, optional): the applied value requires an equal sign (i.e. `--arg=value` ). Defaults to None.
        multiple_values (bool, optional): the argument takes multiple values. Defaults to None.
        number_of_values (int, optional): the argument takes a specific number of values. Defaults to None.
        max_values (int, optional): the argument takes at most X values. Defaults to None.
        min_values (int, optional): the argument takes at min X values. Defaults to None.
        python_type (str, optional): the python type to apply when passing to the wrapped function. Must be one of (“str”, “int”, “float”). Defaults to "str".
        short (str, optional): the short name for the argument, a single character (i.e. `-i` in the CLI would be 'i'),. Defaults to None.
        long (str, optional):  the long name for the argument, a single word (i.e. `--information` in the CLI would be 'information'). Defaults to None.
        long_help (str, optional): the help message to show when a long help message is requested via `--help`. Defaults to None.
        help (str, optional):  the short help message to show during failure or when -h is requested. Defaults to None.
        required (bool, optional): whether this argument is required at run time. Defaults to None.
    """

    def decorator(f):
        if not hasattr(f, "__arguments"):
            f.__arguments = []

        f.__arguments.append({**kwargs})

        @functools.wraps(f)
        def wrapper(*f_args, **f_kwargs):
            return f(*f_args,**f_kwargs)
        return wrapper
    return decorator



def main():
    angreal.main()