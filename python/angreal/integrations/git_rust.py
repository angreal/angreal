"""
Git integration using Rust implementation for better performance.

This module provides a drop-in replacement for the existing git.py module,
using the Rust subprocess implementation for better performance and error handling.
"""
from typing import Optional, Tuple
from pathlib import Path

try:
    from angreal._integrations.git_module import PyGit as _PyGit, git_clone as _git_clone
except ImportError:
    # Fallback if Rust module not available
    _PyGit = None
    _git_clone = None


class GitException(Exception):
    """Git operation failed."""
    pass


class Git:
    """
    Git wrapper using Rust implementation for better performance.

    Compatible with the original Python API for seamless replacement.
    """

    def __init__(self, git_path=None, working_dir=None):
        """
        Initialize Git wrapper.

        Args:
            git_path: Ignored (for compatibility with original API)
            working_dir: Working directory for git operations
        """
        if _PyGit is None:
            raise ImportError("Rust git integration not available")

        self._git = _PyGit(working_dir)
        self.git_path = git_path  # Store for compatibility
        self.working_dir = working_dir or Path.cwd()

    def __call__(self, command: str, *args, **kwargs) -> Tuple[int, bytes, bytes]:
        """
        Execute git command with arguments.

        Returns:
            Tuple of (return_code, stderr, stdout) matching original API
        """
        try:
            return_code, stderr, stdout = self._git(command, list(args), kwargs or {})
            # Original API returns bytes, so encode the strings
            return return_code, stderr.encode('utf-8'), stdout.encode('utf-8')
        except RuntimeError as e:
            raise GitException(str(e))

    def __getattr__(self, name: str):
        """
        Allow method-style calls like git.add('.') for backwards compatibility.

        This preserves the original API where you could call git methods
        directly as attributes.
        """
        def wrapper(*args, **kwargs):
            return self(name, *args, **kwargs)
        return wrapper

    # High-level convenience methods that use the Rust implementation directly
    def init(self, bare=False):
        """Initialize a new repository."""
        try:
            self._git.init(bare)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def add(self, *paths):
        """Add files to staging."""
        try:
            self._git.add(list(paths))
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def commit(self, message, all=False):
        """Create a commit."""
        try:
            self._git.commit(message, all)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def push(self, remote=None, branch=None):
        """Push changes to remote."""
        try:
            self._git.push(remote, branch)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def pull(self, remote=None, branch=None):
        """Pull changes from remote."""
        try:
            self._git.pull(remote, branch)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def status(self, short=False):
        """Get repository status."""
        try:
            result = self._git.status(short)
            return 0, b'', result.encode('utf-8')
        except RuntimeError as e:
            raise GitException(str(e))

    def branch(self, name=None, delete=False):
        """List or create branches."""
        try:
            result = self._git.branch(name, delete)
            return 0, b'', result.encode('utf-8')
        except RuntimeError as e:
            raise GitException(str(e))

    def checkout(self, branch, create=False):
        """Switch branches."""
        try:
            self._git.checkout(branch, create)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))

    def tag(self, name, message=None):
        """Create a tag."""
        try:
            self._git.tag(name, message)
            return 0, b'', b''
        except RuntimeError as e:
            raise GitException(str(e))


def clone(remote: str, destination: Optional[str] = None) -> str:
    """
    Clone a repository.

    Args:
        remote: Git repository URL
        destination: Optional destination directory

    Returns:
        Path to cloned repository
    """
    if _git_clone is None:
        raise ImportError("Rust git integration not available")

    try:
        return _git_clone(remote, destination)
    except RuntimeError as e:
        raise GitException(str(e))


# For backwards compatibility, also provide module-level access
def git_clone(remote: str, destination: Optional[str] = None) -> str:
    """Legacy function name for compatibility."""
    return clone(remote, destination)
