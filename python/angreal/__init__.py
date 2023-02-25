from .angreal import *
import functools

__doc__ = angreal.__doc__
if hasattr(angreal, "__all__"):
    __all__ = angreal.__all__








def command(**kwargs):
    """
    The command decorator, used to register a user defined function as a subcommand for angreal to execute.
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
            name (_type_): _description_
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