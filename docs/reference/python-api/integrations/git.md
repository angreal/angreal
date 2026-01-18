---
title: "Git Integration"
weight: 20
---

# angreal.integrations.git

Git repository operations and version control automation.

## Overview

Angreal's Git integration provides a Python interface for common Git operations. The module wraps the Git command-line tool, offering both convenience methods for common operations and a flexible escape hatch for arbitrary Git commands.

The integration returns structured output with exit codes, stdout, and stderr, making it straightforward to handle both successful operations and error conditions in your automation scripts.

## Prerequisites

Git must be installed and available in your system PATH. The integration does not bundle Git itself.

## Functions

### clone

Clone a Git repository to a local destination.

```python
from angreal.integrations.git import clone

destination = clone(remote, destination=None)
```

**Parameters:**
- `remote` (str): The URL or path of the repository to clone
- `destination` (str, optional): Local path for the cloned repository. If not specified, Git will use the repository name.

**Returns:**
- `str`: The absolute path to the cloned repository

**Example:**
```python
from angreal.integrations.git import clone

# Clone to auto-named directory
path = clone("https://github.com/user/project.git")
print(f"Cloned to: {path}")

# Clone to specific directory
path = clone("https://github.com/user/project.git", "my-local-copy")
```

## Classes

### Git

A wrapper around Git operations for a specific working directory.

```python
from angreal.integrations.git import Git

git = Git(working_dir=None)
```

#### Constructor

```python
Git(working_dir=None)
```

**Parameters:**
- `working_dir` (str, optional): Path to the Git repository. Defaults to the current working directory.

**Raises:**
- `RuntimeError`: If the specified directory does not exist

#### Properties

##### working_dir

Get the working directory path.

```python
@property
def working_dir(self) -> str
```

**Returns:**
- `str`: The absolute path to the working directory

#### Instance Methods

All Git methods return a tuple of `(exit_code, stderr, stdout)` where:
- `exit_code` (int): The command exit code (0 indicates success)
- `stderr` (bytes): Standard error output
- `stdout` (bytes): Standard output

##### init

Initialize a new Git repository.

```python
def init(self, bare=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `bare` (bool, optional): Create a bare repository. Defaults to False.

**Example:**
```python
git = Git("/path/to/new/project")
exit_code, stderr, stdout = git.init()
if exit_code == 0:
    print("Repository initialized")
```

##### add

Stage files for commit.

```python
def add(self, *paths) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `*paths`: Variable number of file paths to stage

**Example:**
```python
# Stage specific files
git.add("README.md", "setup.py")

# Stage all changes
git.add(".")
```

##### commit

Create a commit with the staged changes.

```python
def commit(self, message, all=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `message` (str): The commit message
- `all` (bool, optional): Stage all modified files before committing (equivalent to `git commit -a`). Defaults to False.

**Example:**
```python
# Standard commit
git.add(".")
exit_code, stderr, stdout = git.commit("Add new feature")

# Commit all modified files in one step
git.commit("Quick fix", all=True)
```

##### push

Push commits to a remote repository.

```python
def push(self, remote=None, branch=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `remote` (str, optional): Remote name (e.g., "origin")
- `branch` (str, optional): Branch name to push

**Example:**
```python
# Push to default remote and branch
git.push()

# Push to specific remote and branch
git.push("origin", "main")

# Push current branch to origin
git.push("origin")
```

##### pull

Pull changes from a remote repository.

```python
def pull(self, remote=None, branch=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `remote` (str, optional): Remote name
- `branch` (str, optional): Branch name to pull

**Example:**
```python
# Pull from default remote
git.pull()

# Pull specific branch
git.pull("origin", "develop")
```

##### status

Get the working tree status.

```python
def status(self, short=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `short` (bool, optional): Use short format output. Defaults to False.

**Example:**
```python
exit_code, stderr, stdout = git.status()
print(stdout.decode())

# Short format
exit_code, stderr, stdout = git.status(short=True)
```

##### branch

List or create branches.

```python
def branch(self, name=None, delete=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `name` (str, optional): Branch name to create or delete
- `delete` (bool, optional): Delete the specified branch. Defaults to False.

**Example:**
```python
# List all branches
exit_code, stderr, stdout = git.branch()
print(stdout.decode())

# Create a new branch
git.branch("feature-xyz")

# Delete a branch
git.branch("old-feature", delete=True)
```

##### checkout

Switch branches or restore working tree files.

```python
def checkout(self, branch, create=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `branch` (str): Branch name to switch to
- `create` (bool, optional): Create the branch if it doesn't exist (equivalent to `git checkout -b`). Defaults to False.

**Example:**
```python
# Switch to existing branch
git.checkout("develop")

# Create and switch to new branch
git.checkout("new-feature", create=True)
```

##### tag

Create or list tags.

```python
def tag(self, name, message=None) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `name` (str): Tag name
- `message` (str, optional): Tag message for annotated tags

**Example:**
```python
# Create lightweight tag
git.tag("v1.0.0")

# Create annotated tag
git.tag("v2.0.0", message="Release version 2.0.0")
```

##### execute

Execute an arbitrary Git subcommand.

```python
def execute(self, subcommand, args) -> tuple[int, bytes, bytes]
```

**Parameters:**
- `subcommand` (str): The Git subcommand (e.g., "log", "diff", "stash")
- `args` (List[str]): Arguments for the subcommand

**Example:**
```python
# View commit log
exit_code, stderr, stdout = git.execute("log", ["--oneline", "-5"])
print(stdout.decode())

# Show diff
exit_code, stderr, stdout = git.execute("diff", ["HEAD~1"])

# Stash changes
git.execute("stash", ["push", "-m", "Work in progress"])
```

#### Callable Interface

The Git class supports a callable interface for executing commands with keyword arguments.

```python
git(command, *args, **kwargs) -> tuple[int, bytes, bytes]
```

**Example:**
```python
# These are equivalent
git.execute("log", ["--oneline"])
git("log", "--oneline")

# With keyword arguments (converted to flags)
git("commit", "-m", "Message", amend=True)
```

### GitException

Exception raised for Git-related errors.

```python
from angreal.integrations.git import GitException
```

This exception is raised when attempting to call an unsupported Git command through attribute access.

## Examples

### Basic Repository Workflow

```python
from angreal.integrations.git import Git, clone

# Clone a repository
path = clone("https://github.com/user/project.git", "local-project")

# Work with the repository
git = Git(path)

# Make changes and commit
git.add("README.md")
exit_code, stderr, stdout = git.commit("Update documentation")

if exit_code == 0:
    print("Commit successful")
    git.push("origin", "main")
else:
    print(f"Commit failed: {stderr.decode()}")
```

### Feature Branch Workflow

```python
from angreal.integrations.git import Git

git = Git("/path/to/repo")

# Create and switch to feature branch
git.checkout("feature/new-api", create=True)

# Make changes
git.add(".")
git.commit("Implement new API endpoint")

# Push feature branch
git.push("origin", "feature/new-api")
```

### Release Tagging

```python
from angreal.integrations.git import Git
import angreal

@angreal.command(name="release", about="Create a release tag")
@angreal.argument("version", help="Version number (e.g., 1.2.3)")
def release(version):
    """Tag the current commit as a release."""
    git = Git(".")

    tag_name = f"v{version}"
    message = f"Release version {version}"

    exit_code, stderr, stdout = git.tag(tag_name, message=message)

    if exit_code == 0:
        print(f"Created tag {tag_name}")
        git.push("origin", tag_name)
    else:
        print(f"Failed to create tag: {stderr.decode()}")
```

### Status Checking

```python
from angreal.integrations.git import Git

def check_repo_status():
    """Check and report repository status."""
    git = Git(".")

    exit_code, stderr, stdout = git.status(short=True)
    status_output = stdout.decode().strip()

    if not status_output:
        print("Working tree is clean")
    else:
        print("Pending changes:")
        print(status_output)
```

### Custom Git Commands

```python
from angreal.integrations.git import Git

git = Git(".")

# View recent commits
exit_code, stderr, stdout = git.execute("log", [
    "--oneline",
    "--graph",
    "--decorate",
    "-10"
])
print(stdout.decode())

# Interactive rebase (non-interactive version)
git.execute("rebase", ["--onto", "main", "feature-old", "feature-new"])

# Cherry-pick a commit
git.execute("cherry-pick", ["abc123"])
```

## Error Handling

All Git operations return an exit code as the first element of the tuple. A non-zero exit code indicates an error occurred.

```python
from angreal.integrations.git import Git

git = Git(".")

exit_code, stderr, stdout = git.push("origin", "main")

if exit_code != 0:
    error_message = stderr.decode()
    if "rejected" in error_message:
        print("Push rejected - pull changes first")
    elif "Authentication" in error_message:
        print("Authentication failed - check credentials")
    else:
        print(f"Push failed: {error_message}")
```

## See Also

- [Docker Compose Integration](/angreal/reference/python-api/integrations/docker-compose) - Container orchestration
- [Virtual Environment Integration](/angreal/reference/python-api/integrations/venv) - Python environment management
- [Flox Integration](/angreal/reference/python-api/integrations/flox) - Cross-language environment management
