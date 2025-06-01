"""Virtual environment management using UV."""
import sys
from pathlib import Path
from typing import List, Optional, Union
import functools

from angreal import (
    ensure_uv_installed,
    uv_version,
    create_virtualenv,
    install_packages,
    install_requirements,
    discover_pythons,
    install_python,
)


def venv_required(path, requirements=None):
    """Wrap a function in a virtual environment before execution

    Args:
        path (str): The path to the virtual environment (or where the environment
          should be created if it doesn't exist)
        requirements (_type_, optional): A string containing a single module, a
          list of module names, or a string containing a file path. Defaults to None.
    """
    def decorator(f):
        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            initial_sys_prefix = sys.prefix
            venv = VirtualEnv(path=path, now=True, requirements=requirements)
            venv.install_requirements()
            rv = f(*args, **kwargs)
            sys.prefix = initial_sys_prefix
            return rv
        return wrapper
    return decorator


class VirtualEnv:
    """Virtual environment management using UV."""

    base_path = Path.home() / ".venv"

    def __init__(
        self,
        path: Union[str, Path] = ".venv",
        python: Optional[str] = None,
        requirements=None,
        now=True
    ):
        self.path = Path(path)
        self.python_version = python
        self.requirements = requirements
        self.now = now

        if self.now:
            if not self.exists:
                self._create()

    def _create(self) -> None:
        """Create virtual environment using UV."""
        ensure_uv_installed()
        create_virtualenv(str(self.path), self.python_version)

    def install_requirements(self) -> None:
        """Install requirements set during initialization."""
        if not self.requirements:
            return

        ensure_uv_installed()
        if isinstance(self.requirements, list):
            install_packages(str(self.path), self.requirements)
        elif isinstance(self.requirements, str) and Path(self.requirements).exists():
            install_requirements(str(self.path), self.requirements)
        elif isinstance(self.requirements, str):
            install_packages(str(self.path), [self.requirements])
        else:
            raise TypeError(
                f"requirements should be one of: file, list, or string "
                f"got {type(self.requirements)}"
            )

    def install(self, packages: Union[str, List[str], Path]) -> None:
        """Install packages using UV."""
        ensure_uv_installed()
        if isinstance(packages, (str, Path)) and str(packages).endswith('.txt'):
            install_requirements(str(self.path), str(packages))
        else:
            if isinstance(packages, str):
                packages = [packages]
            install_packages(str(self.path), packages)

    @property
    def exists(self) -> bool:
        """Check if virtual environment exists."""
        return (self.path / "pyvenv.cfg").exists()

    @property
    def python_executable(self) -> Path:
        """Get Python executable path."""
        if sys.platform == "win32":
            return self.path / "Scripts" / "python.exe"
        return self.path / "bin" / "python"

    @staticmethod
    def discover_available_pythons() -> List[tuple[str, str]]:
        """Discover all Python installations on the system."""
        ensure_uv_installed()
        return discover_pythons()

    @staticmethod
    def ensure_python(version: str) -> str:
        """Ensure a specific Python version is available, installing if needed."""
        ensure_uv_installed()
        return install_python(version)

    @staticmethod
    def version() -> str:
        """Get UV version."""
        ensure_uv_installed()
        return uv_version()

    def __str__(self):
        return str(self.path)
