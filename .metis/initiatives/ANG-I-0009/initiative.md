---
id: in-place-template-rendering
level: initiative
title: "In-Place Template Rendering"
short_code: "ANG-I-0009"
created_at: 2026-05-26T15:45:16.727338+00:00
updated_at: 2026-05-26T16:29:50.628796+00:00
parent: ANG-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: S
initiative_id: in-place-template-rendering
---

# In-Place Template Rendering Initiative

## Context **[REQUIRED]**

Today `angreal init <template>` always renders a template's single top-level
templated directory (e.g. `{{ project_name }}/`) as a **new** child directory
inside the current working directory. The project root is therefore always
created by angreal.

A common workflow is to plan a project first — create and `cd` into a folder
(often already a git repo, with notes, an LLM scratchpad, etc.) — and *then*
want to scaffold template content into that existing folder. The current engine
can't do this: it insists on creating the root directory itself.

This initiative adds an **in-place** mode that strips the template's top-level
templated root directory and renders its contents directly into the current
working directory.

## Goals & Non-Goals **[REQUIRED]**

**Goals:**
- Add an `--in-place` / `-i` flag to `angreal init` that renders a template's
  contents into the current directory instead of creating the root directory.
- Reuse existing `--force` semantics for collisions with files already present.
- Keep default (non-in-place) behavior byte-for-byte unchanged.
- Cover the behavior with Rust integration tests and Python functional tests,
  and document it.

**Non-Goals:**
- A per-template `in_place = true` setting in `angreal.toml` (CLI flag only for
  now; can be revisited later).
- Supporting templates with zero or multiple top-level templated directories in
  in-place mode (these will error with a clear message).
- Changing how `init.py` runs or how `angreal.toml` values are persisted, beyond
  pointing them at the new render location.

## Design Decisions (confirmed with maintainer)

1. **Trigger**: CLI flag `--in-place` (`-i`) on `angreal init`. The same
   template works both ways; the caller decides at init time.
2. **Root detection**: Require exactly **one** top-level `{{...}}` directory.
   Strip it and render its contents into cwd. Error clearly if there are zero or
   multiple top-level templated directories.
3. **Collisions**: Reuse `--force` semantics. Without `--force`, error on any
   existing file/dir collision (mirrors today's root-dir existence check). With
   `--force`, overwrite.

## Detailed Design **[REQUIRED]**

### Surface (call chain)
- `crates/angreal/src/builder/mod.rs::add_init_subcommand` (`builder/mod.rs:35`)
  — add the `in_place` flag (`-i` / `--in-place`).
- `crates/angreal/src/lib.rs` init dispatch (`lib.rs:542`) — read
  `is_present("in_place")` and pass it through.
- `crates/angreal/src/init.rs::init` and `::render_template` — thread an
  `in_place: bool` parameter down to `render_dir`.
- `crates/angreal/src/utils.rs::render_dir` (`utils.rs:157`) — core change.

### Core change in `render_dir`
Current behavior: walks the template `src`, and for every directory whose name
starts with `{{` and contains `}}`, renders it into `dst` (cwd); files are added
to Tera only when their relative path starts with `{{`.

In-place behavior:
1. Identify the set of top-level entries under `src` whose names are templated
   (`{{...}}`). Require exactly one directory; otherwise error.
2. Treat that directory as the strip prefix. Render its **children** into `dst`
   directly (i.e. drop the first path segment), rather than creating it.
3. Apply the existing `force` existence check per rendered file/dir against
   `dst` so non-`--force` runs fail before writing on a collision.

The `.angreal` directory currently lives *under* the templated root, so after
stripping it lands directly in cwd — `render_template`'s post-render step that
locates the `.angreal` path and writes `angreal.toml` values must still find it
(it scans `rendered_files` for an entry ending in `.angreal`, which continues to
work since the rendered path is now `<cwd>/.angreal`).

### init.py execution
`init()` chdir's into the rendered `.angreal`'s parent before running `init.py`.
In in-place mode that parent is the current working directory, which is correct.

## Testing Strategy

- **Rust integration** (`crates/angreal/tests/integration/init.rs`): in-place
  render strips the root and writes into a temp cwd; errors on zero/multiple
  top-level templated dirs; `--force` overwrite vs non-force collision error.
- **Python functional** (`py_tests/test_functional.py`): end-to-end `angreal
  init --in-place` against a fixture template, asserting files land in cwd and
  `.angreal/angreal.toml` is written.
- Regression: existing non-in-place tests must remain green unchanged.

## Alternatives Considered **[REQUIRED]**

- **angreal.toml `in_place` setting**: rejected as the sole mechanism because it
  fixes the choice per-template; the same template should scaffold either way.
  May be added later as a default that the flag overrides.
- **Strip all top-level templated dirs / merge**: rejected as ambiguous on
  collisions and inconsistent with the existing single-root convention.

## Implementation Plan **[REQUIRED]**

Proposed task decomposition (pending approval before creating task docs):
1. Core `render_dir` in-place rendering + root detection/stripping (Rust),
   threading the `in_place` param through `init.rs`.
2. CLI wiring: `--in-place`/`-i` flag in `builder/mod.rs` and dispatch in
   `lib.rs`.
3. Tests: Rust integration + Python functional.
4. Documentation: how-to guide + CLI reference update.
