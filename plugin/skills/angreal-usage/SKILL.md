---
name: angreal-usage
description: Consult before running ANY development operation in an angreal project — tests, build, lint, format, type-check, docs, deploy, clean, install, migrate, release. Also for "run an angreal task", "execute angreal command", "discover angreal tasks", "use angreal tree", "what tasks are available", "angreal mcp", "angreal init", "angreal alias", "angreal completion", "verbose angreal output", "ANGREAL_DEBUG", or when chaining tasks, picking the right task for a job, or interpreting an angreal exit code. The operational essentials (top-level CLI surface, exit codes, debug env vars) are already injected by the SessionStart/PreCompact hook — this skill is the deep dive on group nesting, workflow composition, and task selection heuristics.
version: 2.8.7
---

# Using Angreal Tasks (Deep Dive)

The session-start hook already injects the task tree, the "use angreal first" rule, the top-level CLI surface (`tree`, `mcp`, `init`, `alias`, `completion`, `-v`, `--help`), the exit-code table, and the debug env vars. This skill covers what doesn't fit in that always-present preamble: argument syntax, group nesting, workflow composition, and how to choose between tasks.

## Argument Syntax

**Flags** (boolean switches — no value):

```bash
angreal build --release
angreal test rust --unit-only
```

**Value arguments** (both forms work unless the task uses `require_equals`):

```bash
angreal deploy --version v1.2.3
angreal test completion --shell=bash
```

**Multi-value arguments** (repeat the flag):

```bash
angreal compile --file a.txt --file b.txt
```

**Short flags** chain with single-char names:

```bash
angreal build -vj 8        # if -v and -j are both single-char short flags
```

When in doubt, run `angreal <command> --help` — it always shows the exact accepted syntax.

## Task Groups and Nesting

Tasks are organized hierarchically. Two-level nesting is common; deeper nesting works but is rare.

```bash
angreal test rust              # single group
angreal docker compose up      # nested groups
angreal docs build --draft     # group + flag
```

Groups are namespaces, not tasks themselves — `angreal docker` with no subcommand will list the docker group's commands. Use `angreal tree` to see the full tree at a glance.

A single command name can legitimately appear under multiple groups (e.g. `angreal test all` and `angreal docs all`) — Angreal 2.8.5+ keys the registry by full path so this is collision-free.

## Choosing the Right Task

When `angreal tree --long` shows several candidates, the `ToolDescription` blocks include `risk_level` and "When to use / When NOT to use" guidance. Prefer:

1. The task whose **"When to use"** lists the current scenario.
2. **`read_only`** over `safe` over `destructive` when exploring or unsure.
3. The **narrowest** task that does the job (e.g. `test rust --unit-only` over `test all` when you only changed Rust unit code).
4. **Composite tasks** (`test all`, `ci`) when the user asks for "the full check" or before a release.

If two tasks look equally appropriate, the one with the more detailed `ToolDescription` is usually the canonical entry point — task authors put effort into the one they want agents to pick.

## Common Workflow Compositions

### Pre-commit / pre-push

```bash
angreal lint check && angreal test all
```

Chain with `&&` so a failure halts the chain. Each task's non-zero exit propagates naturally.

### Release cut

```bash
angreal test all \
  && angreal build --release \
  && angreal docs build \
  && angreal deploy staging
```

Stop at the first failure. For destructive steps (`deploy`), confirm the user wants to proceed rather than chaining blindly.

### Debugging a failing task

```bash
ANGREAL_DEBUG=true angreal test rust --unit-only -v
```

`ANGREAL_DEBUG=true` enables Angreal's own debug logging (task discovery, registration, argument parsing). The task-internal `-v` flag — if the task author wired one — controls the task's own verbosity. The two are independent.

### Iterating on a single test

If `angreal test python` runs the whole pytest suite but you want a single test, check whether the task accepts a pattern argument (`angreal test python --help`). If not, fall through to the underlying tool (`pytest tests/test_foo.py::test_bar`) — this is one of the rare cases where bypassing angreal is correct.

## When to Bypass Angreal

The "always use angreal" rule has a few legitimate exceptions:

- **Single-file or filter-style invocations** the angreal task doesn't expose (e.g. running one pytest node).
- **Inspection commands** that exist outside the project's automation contract (`git log`, `ls`, `cat`).
- **One-off exploration** the user explicitly asks for ("just run `cargo check` directly").

Default behavior is still: check `angreal tree` first; only bypass when the task genuinely can't express what's needed.
