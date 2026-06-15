# Copyright 2024 Cloacina Contributors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Documentation tasks for Angreal.

The documentation site is built with MkDocs (Material theme) from the markdown
under ``docs/``. Rust/PyO3 API reference pages under ``docs/api/`` are generated
by Plissken (``plissken render``, configured by ``plissken.toml``). This mirrors
the CI pipeline in ``.github/workflows/docs.yaml``.
"""

import shutil
import subprocess
import sys
from pathlib import Path

import angreal  # type: ignore
from angreal.integrations.venv import VirtualEnv

# Project root for accessing docs, etc. (one level up from .angreal)
PROJECT_ROOT = Path(angreal.get_root()).parent

# Build dependencies installed into the isolated docs virtualenv. Keep in sync
# with the "Install MkDocs" step in .github/workflows/docs.yaml.
DOCS_VENV = "angreal-docs-venv"
DOCS_DEPS = ["mkdocs", "mkdocs-material"]

# Define command group
docs = angreal.command_group(name="docs", about="commands for documentation tasks")


def _clean_docs():
    """Remove build artifacts: the MkDocs site/ and generated API docs."""
    cleaned = False
    for target in (PROJECT_ROOT / "site", PROJECT_ROOT / "docs" / "api"):
        if target.exists():
            print(f"Removing {target.relative_to(PROJECT_ROOT)}...")
            shutil.rmtree(target)
            cleaned = True
    if cleaned:
        print("Clean complete!")
    else:
        print("Nothing to clean.")
    return 0


def _render_api_docs():
    """Generate the API reference markdown with Plissken (writes to docs/api)."""
    if shutil.which("plissken") is None:
        print(
            "ERROR: plissken is not installed. Install it with:\n"
            "  curl -fsSL https://raw.githubusercontent.com/colliery-io/"
            "plissken/main/install.sh | bash",
            file=sys.stderr,
        )
        return 1

    print("Generating API documentation with Plissken...")
    try:
        result = subprocess.run(
            ["plissken", "render"],
            cwd=str(PROJECT_ROOT),
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print(f"Failed to render API docs: {e}", file=sys.stderr)
        return e.returncode
    print("API documentation generated in docs/api")
    return result.returncode


@docs()
@angreal.command(
    name="clean",
    about="Clean documentation build artifacts and generated API docs",
    tool=angreal.ToolDescription("""
Clean documentation build output: the MkDocs ``site/`` directory and the
Plissken-generated ``docs/api/`` reference pages.

## When to use
- Before fresh documentation builds
- When build artifacts are stale or corrupted
- During documentation troubleshooting

## When NOT to use
- During normal development
- When incremental builds are sufficient

## Examples
```
angreal docs clean
```
""", risk_level="destructive")
)
def clean():
    """Clean documentation build artifacts and generated API docs."""
    return _clean_docs()


@docs()
@angreal.command(
    name="serve",
    about="Start local documentation server with live reload",
    tool=angreal.ToolDescription("""
Render the API docs with Plissken, then start the MkDocs dev server with live
reload for documentation development.

## When to use
- During documentation writing
- For previewing documentation changes
- When reviewing documentation locally

## When NOT to use
- In production environments
- For final documentation builds (use ``build``)

## Examples
```
angreal docs serve                 # serve on http://127.0.0.1:8000
angreal docs serve --addr 0.0.0.0:8001
```
""", risk_level="safe")
)
@angreal.argument(
    name="addr",
    long="addr",
    help="address:port for the dev server (default 127.0.0.1:8000)",
    required=False,
    takes_value=True,
)
def serve(addr: str = "127.0.0.1:8000"):
    """Serve the MkDocs documentation site locally with regenerated API docs.

    Args:
        addr: address:port to bind the dev server to. Defaults to 127.0.0.1:8000.
    """
    print("=== Setting up documentation ===")

    # Regenerate the API reference first so the served site is current.
    api_result = _render_api_docs()
    if api_result != 0:
        return api_result

    print("\n=== Starting MkDocs server ===")
    print(f"Documentation will be available at http://{addr}")
    print("Press Ctrl+C to stop the server")

    with VirtualEnv(DOCS_VENV, now=True) as venv:
        print("Installing documentation dependencies (mkdocs, mkdocs-material)...")
        venv.install(DOCS_DEPS)
        try:
            result = subprocess.run(
                [venv.python_executable, "-m", "mkdocs", "serve", "--dev-addr", addr],
                cwd=str(PROJECT_ROOT),
                check=True,
            )
            return result.returncode
        except subprocess.CalledProcessError as e:
            print(f"MkDocs server failed: {e}", file=sys.stderr)
            return e.returncode


@docs()
@angreal.command(
    name="build",
    about="Build production documentation site with generated API docs",
    tool=angreal.ToolDescription("""
Render the API docs with Plissken, then build the MkDocs site into ``site/``.
By default the build is strict (warnings fail the build), matching CI.

## When to use
- For production deployments
- Before releasing documentation
- For final documentation review

## When NOT to use
- During active documentation writing (use ``serve`` instead)
- For quick local previews

## Examples
```
angreal docs build              # strict production build (warnings fail)
angreal docs build --no-strict  # tolerate warnings while drafting
```
""", risk_level="safe")
)
@angreal.argument(
    name="no_strict",
    long="no-strict",
    help="do not fail the build on warnings",
    required=False,
    takes_value=False,
    is_flag=True,
)
def build(no_strict: bool = False):
    """Build the MkDocs documentation site with regenerated API docs.

    Args:
        no_strict: If True, warnings do not fail the build. Defaults to False
            (strict, matching CI).
    """
    print("=== Building documentation site ===")

    # Clean previous artifacts for a reproducible build.
    clean_result = _clean_docs()
    if clean_result != 0:
        return clean_result

    # Regenerate the API reference.
    api_result = _render_api_docs()
    if api_result != 0:
        return api_result

    print("\nBuilding MkDocs site...")
    cmd_tail = ["-m", "mkdocs", "build"]
    if no_strict:
        print("Strict mode disabled (warnings will not fail the build)")
    else:
        cmd_tail.append("--strict")
        print("Strict mode enabled (warnings fail the build)")

    with VirtualEnv(DOCS_VENV, now=True) as venv:
        print("Installing documentation dependencies (mkdocs, mkdocs-material)...")
        venv.install(DOCS_DEPS)
        try:
            result = subprocess.run(
                [venv.python_executable, *cmd_tail],
                cwd=str(PROJECT_ROOT),
                check=True,
            )
            if result.returncode == 0:
                print("\n=== Build complete ===")
                print(f"Documentation site built in {PROJECT_ROOT}/site")
            return result.returncode
        except subprocess.CalledProcessError as e:
            print(f"Failed to build documentation: {e}", file=sys.stderr)
            return e.returncode
