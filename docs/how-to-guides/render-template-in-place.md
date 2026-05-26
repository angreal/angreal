---
title: Render a Template In Place
weight: 35
---

# Render a Template In Place

By default, `angreal init` renders a template's single top-level directory (for
example `{{ project_name }}/`) as a **new** project root inside your current
directory. This guide shows how to render a template's contents *directly into
the current directory* instead — useful when you have already created and
entered a folder (often an existing git repository or a directory holding
planning notes) and want to scaffold a template into it.

## Prerequisites

- A template whose layout has exactly **one** top-level templated directory
  (the usual convention, e.g. `{{ project_name }}/`).
- An existing directory you want to populate.

## Render in place

From inside the directory you want to populate:

```bash
angreal init <template> --in-place
```

`--in-place` (short flag `-i`) strips the template's top-level directory and
renders its contents — including the `.angreal/` directory — directly into the
current working directory.

### Example

Given a template laid out like this:

```text
my-template/
├── angreal.toml
└── {{ project_name }}/
    ├── .angreal/
    │   └── init.py
    ├── README.md
    └── src/
        └── main.py
```

Running:

```bash
mkdir my-project && cd my-project
angreal init ../my-template --in-place
```

produces:

```text
my-project/
├── .angreal/
│   └── init.py
├── README.md
└── src/
    └── main.py
```

Note that the `{{ project_name }}` directory is **not** created — its contents
land directly in `my-project/`.

## Handling existing files

In-place rendering reuses the standard `--force` semantics. If any file or
directory the template would write already exists in your current directory,
the command aborts without making changes:

```bash
$ angreal init ../my-template --in-place
README.md already exist(s) in /path/to/my-project. Will not proceed unless `--force`/force=True is used.
```

Pass `--force` to overwrite the conflicting files:

```bash
angreal init ../my-template --in-place --force
```

## Requirements and errors

In-place rendering requires the template to have exactly one top-level templated
directory. If a template has **none** or **more than one**, angreal cannot
determine which directory to strip and exits with an error:

```bash
$ angreal init ../bad-template --in-place
--in-place requires exactly one top-level templated directory, but found 2 ...
```

If you hit this, either render the template normally (without `--in-place`), or
restructure the template so it has a single top-level templated directory.

!!! info
    For a deeper explanation of how Angreal resolves and renders templates, see
    [angreal init Behavior](../explanation/angreal_init_behaviour.md).
