from .angreal import *


__doc__ = angreal.__doc__
if hasattr(angreal, "__all__"):
	__all__ = angreal.__all__


def main():
	angreal.main()