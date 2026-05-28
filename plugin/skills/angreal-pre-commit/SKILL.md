---
name: angreal-pre-commit
description: This skill should be used when the user asks to "add a pre-commit hook for angreal", "run angreal tasks before commit", "run angreal on pre-push", "set up pre-commit for tests/lint", "configure .pre-commit-config.yaml for angreal", "make tests run automatically before commit", or needs guidance on wiring [pre-commit](https://pre-commit.com/) to invoke angreal tasks (test, lint, format, etc.) at git lifecycle stages.
version: 2.8.7
---

# Pre-commit Integration for Angreal

The [`angreal/pre-commit-angreal`](https://github.com/angreal/pre-commit-angreal) repo provides a [pre-commit](https://pre-commit.com/) hook that runs angreal tasks at git lifecycle stages (pre-commit, pre-push, pre-merge-commit, etc.). Use this to make a project's angreal tasks the enforced quality gate before commits or pushes — no separate Makefile / npm script / shell wrapper.

## Minimal Setup

Add to the project's `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/angreal/pre-commit-angreal
    rev: v0.1.0   # check the repo for the current tag — bump as needed
    hooks:
      - id: angreal
        args:
          - "test rust --unit-only"
          - "test python"
```

Then install the git hook once per clone:

```bash
pre-commit install
```

Each entry in `args` is a **full angreal command string** (everything you'd type after `angreal`). They run serially and fail fast on the first non-zero exit, so order them cheapest-first (lint before tests, unit before integration).

## Pinning the Angreal Version

By default the hook installs the latest angreal. Pin it for reproducibility:

```yaml
hooks:
  - id: angreal
    additional_dependencies: ["angreal==2.8.7"]
    args:
      - "test rust --unit-only"
```

This matters when the project's tasks depend on Angreal features added in a specific release (e.g. ToolDescription, `--in-place`, `required_version`). Pin to the floor version your `.angreal/` files need.

## Different Tasks at Different Stages

```yaml
hooks:
  - id: angreal
    alias: angreal-pre-commit
    args: ["test rust --unit-only"]
    stages: [pre-commit]
  - id: angreal
    alias: angreal-pre-push
    args: ["test all"]
    stages: [pre-push]
```

Common split:
- **pre-commit**: fast checks only (lint, format, unit tests on changed components) — anything slower than ~10s frustrates contributors and gets bypassed.
- **pre-push**: full suite (`angreal test all`, integration tests, doc build) — runs less often, so the cost is acceptable.

Install both stages: `pre-commit install --hook-type pre-commit --hook-type pre-push`.

## Why Use This Instead of Plain pre-commit Hooks

- **One source of truth.** Tasks already exist in `.angreal/`; this just calls them. No reimplementation in YAML.
- **Same command works locally and in the hook.** `angreal test python` runs the same way whether you type it, the hook invokes it, or CI runs it — no behavior drift between contexts.
- **Tasks already encode project conventions.** Correct flags, working directory, environment setup. A hand-written hook would have to re-derive all of that.

## When NOT to Use This

- The check you want is genuinely outside angreal's scope (whitespace fixers, secret detectors, file-format validators) — use the dedicated pre-commit hooks for those (`trailing-whitespace`, `detect-secrets`, etc.) and let pre-commit-angreal handle only the project-specific tasks.
- The angreal task is long-running (>30s) and would block every commit — move it to `pre-push` or CI, not `pre-commit`.

## Tracking the Current rev

The skill pins `v0.1.0` above (initial tag, cut 2026-05). For newer revs, run `gh api repos/angreal/pre-commit-angreal/tags --jq '.[].name'` or check the [tags page](https://github.com/angreal/pre-commit-angreal/tags). `pre-commit autoupdate` will also bump `rev:` to the latest tag automatically.
