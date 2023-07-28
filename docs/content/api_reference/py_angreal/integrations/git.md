---
title: Git
---

Angreal provides a very thin layer to an available git binary on the operating system. It is not
an interface to libgit or libgit2 but an interface to a command line interface.

#### Git(**git_path**: str=*None*,**working_dir**: str=*None*)
> a hyper light weight wrapper for a git binary. By default will attempt to find the first `git` on syspath to utilize.

```python
from angreal.integrations.git import Git

g = Git()
git.init()
git.add("file")
git.commit("-am","commit message")

```
### Args:
- git_path (str, optional): path to git to use. If None, defaults to the first available on sys.path. Defaults to None
- working_dir (str,optional): path for working directory of git commands. If None, defaults to '.'. Defaults to None
---
