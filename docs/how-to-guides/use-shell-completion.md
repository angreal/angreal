---
title: Use Shell Completion
weight: 20
---

# Use Shell Completion

This guide walks you through enabling and managing shell completion so that
`angreal` commands, template names, task names, and arguments complete on `[TAB]`.

Shell completion helps you:

- Complete `angreal` commands automatically
- Get template suggestions when using `angreal init`
- Complete task names within Angreal projects
- Complete command arguments

Supported shells: **bash** and **zsh**.

## 1. Install completion

The first time you run Angreal, it auto-installs completion for your current
shell. If you need to install (or reinstall) it manually, run:

```bash
# Install for the current shell (auto-detected from $SHELL)
angreal completion install

# Or install for a specific shell
angreal completion install bash
angreal completion install zsh
```

The `[shell]` argument is optional; when omitted, Angreal detects your shell from
the `$SHELL` environment variable.

## 2. Activate completion in your current shell

Installation writes the completion script, but your already-open shell session
won't pick it up until it re-reads its startup file. Restart your shell, or
reload it:

```bash
# bash
source ~/.bashrc

# zsh
source ~/.zshrc
```

## 3. Verify it's working

Check the installation status:

```bash
angreal completion status
```

Then try completion in a shell:

```bash
# Type 'angreal ' and press TAB to see available commands
angreal [TAB]

# Type 'angreal init ' and press TAB to see available templates
angreal init [TAB]

# Inside an Angreal project, TAB completes project task names
angreal [TAB]
```

## Uninstall completion

To remove the completion script:

```bash
# Uninstall for the current shell (auto-detected)
angreal completion uninstall

# Or for a specific shell
angreal completion uninstall bash
angreal completion uninstall zsh
```

## Disable auto-install (e.g. in CI)

Angreal auto-installs completion on first run. To prevent that — for example in
a CI environment where you don't want it modifying shell startup files — set the
`ANGREAL_NO_AUTO_COMPLETION` environment variable before running Angreal:

```bash
export ANGREAL_NO_AUTO_COMPLETION=1
```

When this variable is set, the first-run auto-install is skipped. You can still
install completion explicitly with `angreal completion install`.

## Related Documentation

- [Create Tasks](/angreal/how-to-guides/create-a-task) - How to create tasks with completion support
- [Create Templates](/angreal/how-to-guides/create-templates) - How to create templates with completion support
