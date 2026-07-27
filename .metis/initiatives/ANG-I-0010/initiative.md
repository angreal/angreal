---
id: oci-support
level: initiative
title: "OCI Support: Template Targets and Plugin Integration"
short_code: "ANG-I-0010"
created_at: 2026-07-24T00:00:00.000000+00:00
updated_at: 2026-07-24T00:00:00.000000+00:00
parent: ANG-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: true
estimated_complexity: M
initiative_id: oci-support
---

# OCI Support Initiative

## Context **[REQUIRED]**

A feature request — "We should support OCI" — asks angreal to speak
[OCI](https://opencontainers.org/) (Open Container Initiative) registries. In
context this means two related but distinct capabilities:

1. **OCI template targets.** Let `angreal init` consume a template packaged and
   stored as an OCI artifact in a registry (GHCR, Docker Hub, ECR, self-hosted
   Zot/Harbor, …). Registries have become a general-purpose artifact
   distribution mechanism (Helm charts, WASM modules, Homebrew bottles all ship
   as OCI artifacts), and templates fit the same model: versioned, content-
   addressed, access-controlled, cache-friendly.

2. **An OCI plugin interface.** Expose basic OCI registry operations to task
   authors as `angreal.integrations.oci`, mirroring the existing
   `angreal.integrations.{git,venv,flox,docker}` modules, so tasks can pull and
   push artifacts, list tags, and authenticate against registries.

Both halves share one Rust core: an OCI client wrapper. `init` consumes its pull
path; the Python binding exposes the fuller surface.

## Goals & Non-Goals **[REQUIRED]**

**Goals:**
- `angreal init oci://<registry>/<repo>:<tag>` resolves, pulls, caches, and
  renders a template artifact, reusing the existing render pipeline unchanged.
- A pure-Rust OCI core (`crates/angreal/src/integrations/oci/`) built on the
  `oci-client` crate: pull, push, list tags, and registry auth.
- Define an **angreal template artifact convention** (a gzipped tar of the
  template directory carried in an OCI layer with a dedicated media type) and a
  **push** path so authors can publish templates, not only consume them.
- Expose `angreal.integrations.oci` with a minimal, focused surface:
  `pull`, `push`, `list_tags` (`tags`), and `login`.
- Registry auth: anonymous for public repos, Basic (username/password or token),
  and Docker `config.json` credentials where present.
- Default (non-`oci://`) `init` behavior remains byte-for-byte unchanged.
- Cover with Rust unit/integration tests, Python functional tests, and docs.

**Non-Goals:**
- Running a container / talking to a Docker daemon — that is the separate
  existing `angreal.integrations.docker` (Bollard) surface. This initiative is
  registry/artifact operations, daemonless.
- A rich registry-management surface (manifest inspect, cross-registry copy, tag
  delete, referrers/signatures). May follow later; not in the initial cut.
- Cosign/Notation signature verification of pulled artifacts (future hardening).
- Publishing angreal's own official templates as OCI artifacts (separate ops
  work once the mechanism exists).

## Design Decisions (confirmed with maintainer)

1. **Backend — pure-Rust `oci-client`.** Bundle the `oci-client` crate rather
   than shelling out to `oras`/`skopeo`/`docker`. This matches angreal's
   self-contained philosophy (bundled `git2`/libgit2, bundled `uv`) — no
   external binary required at runtime. Trade-off: `oci-client` is async
   (tokio), so this introduces angreal's first async dependency, bridged to the
   existing synchronous call paths with a `block_on` on a small runtime.
2. **Scope — pull + push.** Define the template artifact convention and provide
   a push path so the authoring loop is closed end-to-end, not pull-only.
3. **Plugin surface — minimal.** `pull` / `push` / `tags` / `login`. Expand
   later based on real demand rather than speculatively.

## Detailed Design **[REQUIRED]**

### Shared Rust core — `crates/angreal/src/integrations/oci/mod.rs`
A thin synchronous wrapper over `oci-client` (`oci-client = "0.17"`), following
the shape of the `git`/`flox`/`uv` integration modules:

- `Oci` (or free functions) with:
  - `pull_artifact(reference, dest, auth) -> Result<PathBuf>` — pull the
    manifest, fetch the template layer(s), unpack the gzipped tar into `dest`.
  - `push_artifact(reference, src_dir, auth) -> Result<()>` — tar+gzip
    `src_dir`, assemble a manifest with the angreal template media type, push
    config + layer + manifest.
  - `list_tags(repository, auth) -> Result<Vec<String>>`.
  - Auth resolution: `RegistryAuth::Anonymous`, `RegistryAuth::Basic`, and a
    helper that reads Docker `~/.docker/config.json` credentials.
- Async is contained here: create a current-thread tokio runtime and
  `block_on` each op so all public methods are synchronous to the rest of
  angreal. (Add `tokio = { version = "1", features = ["rt", "macros"] }`.)

**Template artifact convention (v1):**
- Config media type: `application/vnd.angreal.template.config.v1+json`
  (small JSON: angreal version, template name, created-at).
- Layer media type: `application/vnd.angreal.template.layer.v1.tar+gzip`
  (gzipped tar of the template directory — the same tree `init` renders).
- Artifact manifest per the OCI image-manifest spec so any OCI-compliant
  registry accepts it and `oras`/`skopeo` can interoperate.

### `init` integration — `crates/angreal/src/init.rs`
- `get_scheme` (`init.rs:115`): short-circuit `oci://` **before** git-url
  parsing (the same pattern as the existing Windows-path short-circuit), class
  returning `"oci"`.
- `init` dispatch (`init.rs:40`): add an `"oci"` arm calling a new
  `handle_oci_template(reference, &angreal_home)` that pulls into a cache path
  under `~/.angrealrc/oci/<registry>/<repo>/<tag>/` and returns it, after which
  the existing `render_template` path runs unchanged.
- Caching mirrors git templates: reuse the cached unpack if present (content is
  content-addressed by digest; re-pull is cheap and can refresh on tag).

### Python binding — `crates/angreal/src/python_bindings/integrations/oci.rs`
- New `#[pyclass] Oci` + `#[pymodule] oci` wrapping the core, registered in
  `python_bindings/integrations/mod.rs` and added to `sys.modules` as
  `angreal.integrations.oci` (exactly as `git`/`venv`/`flox` are wired at
  `python_bindings/integrations/mod.rs:31`).
- Surface: `Oci.pull(reference, dest=None)`, `Oci.push(reference, path)`,
  `Oci.tags(repository)`, `Oci.login(registry, username, password)`.
- Errors surface as Python exceptions via the existing error-formatting path.

### Dependencies
- `oci-client = "0.17"` and `tokio` (rt + macros) added to
  `crates/angreal/Cargo.toml`. Confirm `extension-module` feature interaction
  and musl build (CI already builds musl wheels).
- **TLS backend — reuse the existing one, do not add a second.** angreal already
  vendors OpenSSL (`openssl = { features = ["vendored"] }`) and `reqwest`/`git2`
  use native-tls against it; this already builds static on musl in CI. Configure
  `oci-client` with its **default `native-tls`** feature so it rides the same
  vendored OpenSSL — one TLS backend, nothing new on musl. Do **not** enable
  `oci-client`'s `rustls-tls` feature, which would pull rustls in alongside the
  existing OpenSSL. Task 1 only needs to confirm `oci-client` (default features)
  compiles against the vendored OpenSSL under musl.

## Alternatives Considered **[REQUIRED]**

- **Shell out to `oras`/`skopeo`/`docker`.** Rejected: adds a hard runtime
  dependency users must install and breaks the self-contained model angreal
  holds elsewhere (git2, uv). Pure-Rust keeps single-binary distribution.
- **Reuse the existing Docker (Bollard) integration.** Rejected: that is a
  Docker *daemon* client for containers, not daemonless *registry/artifact*
  operations, and would force a running Docker daemon for template pulls.
- **Pull-only (publish via external `oras`).** Rejected in favor of shipping a
  push path so template authoring is a first-class, self-contained workflow.

## Implementation Plan **[REQUIRED]**

Proposed task decomposition (pending maintainer approval before creating task
docs):

1. **OCI Rust core** — `integrations/oci/` module: `oci-client` + tokio bridge,
   pull/push/list_tags, auth (anonymous/basic/docker-config), the template
   artifact media-type convention, and unit tests. Add deps to `Cargo.toml`.
2. **`init` OCI template targets** — `oci://` scheme detection + dispatch +
   cache path + `handle_oci_template`, wiring into the existing render pipeline;
   Rust integration tests against a local registry (Zot/`registry:2`).
3. **Python `angreal.integrations.oci` binding** — pyclass/pymodule for
   pull/push/tags/login, module registration, Python functional tests.
4. **Documentation** — how-to (consume + publish an OCI template), integration
   reference for `angreal.integrations.oci`, and a note on the artifact
   convention. Update docs nav.

## Open Questions / Risks
- **musl + TLS**: CI builds musl wheels. Reuse the existing vendored-OpenSSL
  native-tls backend for `oci-client` (its default) rather than adding rustls —
  one TLS stack, already proven on musl. Confirm the compile early.
- **macOS TLS (RESOLVED)**: `native-tls` resolves to Apple SecureTransport on
  macOS (not the vendored OpenSSL). Verified working: the `ANGREAL_OCI_HTTPS_SMOKE`
  integration test does a live anonymous `list_tags` over HTTPS against Docker
  Hub and passes on macOS (SecureTransport) locally. CI runs this smoke on the
  macOS job every run (and on Linux, exercising the OpenSSL HTTPS path); the
  localhost round-trip tests only cover HTTP.
- **tokio footprint**: first async dep; keep it feature-minimal (`rt`,
  `macros`) and confined to the oci module.
- **Registry test harness**: integration tests need a throwaway registry.
  Prefer spinning `ghcr.io/project-zot/zot` or `registry:2` in CI (Docker is
  already available for the docker integration tests) with a skip-if-unavailable
  guard for local runs.
