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
    get_venv_activation_info,
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
            venv = VirtualEnv(path=path, now=True, requirements=requirements)
            venv.install_requirements()

            # Activate the virtual environment
            venv.activate()
            try:
                rv = f(*args, **kwargs)
            finally:
                # Always deactivate
                venv.deactivate()

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
        # Convert to Path and resolve to absolute path
        self.path = Path(path).resolve()
        self.python_version = python
        self.requirements = requirements
        self.now = now

        # Activation state tracking
        self._is_activated = False
        self._original_prefix = None
        self._original_path = None
        self._original_real_prefix = None

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

    def activate(self) -> None:
        """Activate the virtual environment in the current Python process."""
        if self._is_activated:
            return

        # Ensure the venv exists
        if not self.exists:
            raise RuntimeError(f"Virtual environment does not exist at {self.path}")

        # Get activation info from Rust
        activation_info = get_venv_activation_info(str(self.path))

        # Save original state
        self._original_prefix = sys.prefix
        self._original_path = sys.path.copy()
        if hasattr(sys, 'real_prefix'):
            self._original_real_prefix = sys.real_prefix

        # Update sys.prefix to the virtual environment
        sys.prefix = activation_info.venv_prefix

        # Set sys.real_prefix to the original prefix (for compatibility)
        if not hasattr(sys, 'real_prefix'):
            sys.real_prefix = self._original_prefix

        # Prepend venv's site-packages to sys.path
        site_packages = activation_info.site_packages

        # Remove any existing site-packages paths to ensure venv takes precedence
        sys.path.copy()
        sys.path = [p for p in sys.path if 'site-packages' not
                     in p or site_packages in p]

        # Insert the venv site-packages at the beginning if not already there
        if site_packages not in sys.path:
            sys.path.insert(0, site_packages)

        # For Windows, we may also need to add the Scripts directory to PATH
        # But we don't modify os.environ['PATH'] here to avoid side effects

        self._is_activated = True

    def deactivate(self) -> None:
        """Restore original Python environment."""
        if not self._is_activated:
            return

        # Restore original state
        if self._original_prefix is not None:
            sys.prefix = self._original_prefix

        if self._original_path is not None:
            sys.path = self._original_path.copy()

        # Restore or remove sys.real_prefix
        if self._original_real_prefix is not None:
            sys.real_prefix = self._original_real_prefix
        elif hasattr(sys, 'real_prefix'):
            delattr(sys, 'real_prefix')

        self._is_activated = False

    def __enter__(self):
        """Context manager support."""
        self.activate()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager support."""
        self.deactivate()
        return False

    def __str__(self):
        return str(self.path)
