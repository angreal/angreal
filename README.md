# Angreal
[![image](https://img.shields.io/pypi/v/angreal.svg)](https://pypi.python.org/pypi/angreal)
![PyPI - Downloads](https://img.shields.io/pypi/dm/angreal)
[![image](https://img.shields.io/pypi/l/angreal.svg)](https://pypi.python.org/pypi/angreal)
[![Angreal Tests](https://github.com/angreal/angreal/actions/workflows/test.yaml/badge.svg?branch=main)](https://github.com/angreal/angreal/actions/workflows/test.yaml)
[![Angreal Docs Deploy](https://github.com/angreal/angreal/actions/workflows/docs.yaml/badge.svg)](https://github.com/angreal/angreal/actions/workflows/docs.yaml)
[![Angreal Release](https://github.com/angreal/angreal/actions/workflows/release.yaml/badge.svg?event=release)](https://github.com/angreal/angreal/actions/workflows/release.yaml)
---
[Docs are available here.](https://angreal.github.io/angreal/)

## Angreal is meant to:
- allow the consistent creation of projects
- provide consistent methods for interacting with projects

### Quick Start

1.  Install via `pip`
2.  Initialize a project from a template
3.  Use the template

```bash
$: pip install 'angreal>=2' #pip install angreal will also work
$: angreal init https://github.com/angreal/python
```
---


## What is it?

Angreal is an attempt to solve two problems that I was running into in
both my personal and professional life as a data scientist and software
developer. I do things often enough that they needed automation, I
don\'t do things so often that I remember all of the steps/commands I
might need to get them done. Angreal solves this problem by allowing me
to remember by forgetting : I only have to remember the command to do
something not the actual steps to complete the task.

### How does it solve these challenges ?

Angreal provides a way to template the structure of projects and a way
of executing methods for interacting with that project in a consistent
manner. These methods (called tasks) travel with the project so while
templated initially, they\'re customizable to the project - allowing some
level of flexibility in how a task functions between projects.

### Why 2.0 ?

The original angreal was built on top of a number of python modules that
were under active development and used by a number of other projects.
The nature of the application itself meant that core application found
itself in dependency hell regularly - and became rather annoying to use.
The 2.0.0 release is a complete rewrite that uses
[Rust](https://www.rust-lang.org/) to provide a compiled binary with the goal that it will
require no external python dependencies.
