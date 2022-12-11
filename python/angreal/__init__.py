from .angreal import *


__doc__ = angreal.__doc__
if hasattr(angreal, "__all__"):
    __all__ = angreal.__all__


def command(name=None, about="", long_help="", **attrs):
    """
    The command decorator, used to register a user defined function as a subcommand for angreal to execute.
    """
    _wrapped = None

    if callable(name):
        _wrapped = name
        name = _wrapped.__name__.lower().replace("_", "-")


    def decorator(f):

        if not hasattr(f, "__arguments"):
            f.__arguments = []
                        
                        
        angreal.Command(name=name, about=about, long_about=f.__doc__, func=f)

        for arg in f.__arguments :
            Arg(**{**arg, **dict(command_name=name)})
        
        return f

    if _wrapped is not None:
        return decorator(_wrapped)

    return decorator


def argument(name, **kwargs):
    """Register an argument as part of a command

    Args:
            name (_type_): _description_
    """

    def decorator(f):
        keyword_args = kwargs.copy()

        if not hasattr(f, "__arguments"):
            f.__arguments = []

        f.__arguments.append({**dict(name=name), **keyword_args})

        return f

    return decorator


def main():
    angreal.main()
