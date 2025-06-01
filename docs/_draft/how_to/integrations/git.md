---
title: "Git Integration"
weight: 1
---

# angreal.integrations.git

Programmatic access to git operations using a high-performance Rust backend.

## Classes

### Git

A high-performance wrapper for git operations powered by Rust and libgit2.

```python
from angreal.integrations.git import Git

git = Git()
```

#### Constructor

```python
Git(working_dir=None)
```

**Parameters:**
- `working_dir` (str, optional): The working directory for git operations. Defaults to current directory.

**Raises:**
- `RuntimeError`: If the specified working_dir doesn't exist

#### Methods

##### \_\_call\_\_

Execute a git command directly.

```python
__call__(command, *args, **kwargs)
```

**Parameters:**
- `command` (str): The git subcommand to run (e.g., 'init', 'add', 'commit')
- `*args`: Positional arguments for the command
- `**kwargs`: Options/flags for the command

**Returns:**
- `tuple`: (return_code, stderr, stdout) from the completed command

**Raises:**
- `GitException`: If the git command returns a non-zero exit status

**Example:**
```python
git = Git()
return_code, stderr, stdout = git('status')
```

##### Dynamic Method Calls

The Git class supports dynamic method calls for git subcommands:

```python
git = Git()
git.add('.')
git.commit(m="Initial commit")
git.push('origin', 'main')
```

This is equivalent to:
```bash
git add .
git commit -m "Initial commit"
git push origin main
```

## Exceptions

### GitException

Raised when a git command fails with a non-zero exit status.

```python
class GitException(Exception)
```

## Examples

### Basic Usage

```python
from angreal.integrations import Git

# Initialize Git in current directory
git = Git()

# Add all files
git.add('.')

# Commit with message
git.commit(m="My commit message")

# Check status
return_code, stderr, stdout = git.status()
print(stdout.decode())
```

### Working with Different Directories

```python
from angreal.integrations import Git

# Work in a specific directory
git = Git(working_dir="/path/to/repo")

# Clone a repository
git.clone("https://github.com/user/repo.git")

# Create and checkout new branch
git.checkout(b="feature-branch")
```

### Error Handling

```python
from angreal.integrations import Git, GitException

git = Git()

try:
    # Try to commit without staging
    git.commit(m="This will fail")
except GitException as e:
    print(f"Git error: {e}")
```

## See Also

- [How to Use Git Integration](/how-to-guides/use-git-integration) - Practical guide
- [Utils Module](/reference/python-api/utils) - Other utility functions
