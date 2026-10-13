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

The documentation site is an Astro Starlight site in ``docs/``, with the
Colliery theme starlight-aurora. It needs Node 22 or later. The API reference
pages under ``docs/src/content/docs/reference/api/`` are rendered by Plissken
(``plissken render``, configured by ``plissken.toml``). They are not in git:
CI renders them before each build (``.github/workflows/docs.yaml``), and
these tasks render them locally with the same Plissken release.
"""

import hashlib
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
from pathlib import Path

import angreal  # type: ignore

# Project root for accessing docs, etc. (one level up from .angreal)
PROJECT_ROOT = Path(angreal.get_root()).parent
DOCS_ROOT = PROJECT_ROOT / "docs"

# Keep in sync with PLISSKEN_VERSION in .github/workflows/docs.yaml.
PLISSKEN_VERSION = "0.2.1"
# sha256 of plissken-<version>-<target>.tar.gz, from the GitHub release.
PLISSKEN_SHA256 = {
    "aarch64-apple-darwin":
        "e6cf08596db58fd0af49990b03bb1624113e680958a9c2858b74b4403f6430ce",
    "x86_64-apple-darwin":
        "dd8da9b293dc8a812e77ec21755f5337cde3ae29b9a36f1fe64653cad52d667c",
    "aarch64-unknown-linux-gnu":
        "035fcecc5ff30bd22d3c0598dc3c8df0d86b0636aec0702a41635e1e4bcf432f",
    "x86_64-unknown-linux-gnu":
        "bf41f2fbc6f375552e29cf56a78b675cf857a1fa689c9905a5caba109ad292bf",
}
PLISSKEN_URL = "https://github.com/colliery-io/plissken/releases/download"
# The downloaded binary; target/ is not in git.
PLISSKEN_BIN = PROJECT_ROOT / "target" / "plissken" / PLISSKEN_VERSION / "plissken"
API_ROOT = DOCS_ROOT / "src" / "content" / "docs" / "reference" / "api"

# Define command group
docs = angreal.command_group(name="docs", about="commands for documentation tasks")


def _clean_docs():
    """Remove build artifacts: docs/dist and the Astro cache."""
    cleaned = False
    for target in (DOCS_ROOT / "dist", DOCS_ROOT / ".astro"):
        if target.exists():
            print(f"Removing {target.relative_to(PROJECT_ROOT)}...")
            shutil.rmtree(target)
            cleaned = True
    if cleaned:
        print("Clean complete!")
    else:
        print("Nothing to clean.")
    return 0


def _plissken_target():
    """The release target of this machine, or None."""
    machine = platform.machine().lower()
    arch = {"arm64": "aarch64", "aarch64": "aarch64",
            "x86_64": "x86_64", "amd64": "x86_64"}.get(machine)
    system = {"Darwin": "apple-darwin",
              "Linux": "unknown-linux-gnu"}.get(platform.system())
    if arch and system:
        return f"{arch}-{system}"
    return None


def _plissken():
    """Path to Plissken PLISSKEN_VERSION, or None after an error message.

    Downloads the release binary into target/ on the first run and checks its
    sha256. Set PLISSKEN to the path of a plissken PLISSKEN_VERSION binary to
    use that one instead.
    """
    override = os.environ.get("PLISSKEN")
    if override:
        if not Path(override).is_file():
            print(f"ERROR: PLISSKEN={override} is not a file", file=sys.stderr)
            return None
        print(f"Using PLISSKEN={override}; it must be plissken {PLISSKEN_VERSION}.")
        return override
    if PLISSKEN_BIN.is_file():
        return str(PLISSKEN_BIN)

    target = _plissken_target()
    if target not in PLISSKEN_SHA256:
        print(
            f"ERROR: no plissken {PLISSKEN_VERSION} download for "
            f"{platform.system()} {platform.machine()}. Install it with\n"
            f"  cargo install plissken --version {PLISSKEN_VERSION} --locked\n"
            "and set PLISSKEN to the path of the binary.",
            file=sys.stderr,
        )
        return None

    asset = f"plissken-{PLISSKEN_VERSION}-{target}.tar.gz"
    url = f"{PLISSKEN_URL}/v{PLISSKEN_VERSION}/{asset}"
    print(f"Downloading plissken {PLISSKEN_VERSION} ({asset})...")
    try:
        with urllib.request.urlopen(url, timeout=60) as response:
            data = response.read()
    except OSError as e:
        print(f"ERROR: could not download {url}: {e}", file=sys.stderr)
        return None
    digest = hashlib.sha256(data).hexdigest()
    if digest != PLISSKEN_SHA256[target]:
        print(f"ERROR: sha256 of {asset} is {digest}, expected "
              f"{PLISSKEN_SHA256[target]}", file=sys.stderr)
        return None

    PLISSKEN_BIN.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        archive = Path(tmp) / asset
        archive.write_bytes(data)
        with tarfile.open(archive) as tar:
            member = tar.extractfile("plissken")
            if member is None:
                print(f"ERROR: no plissken binary in {asset}", file=sys.stderr)
                return None
            PLISSKEN_BIN.write_bytes(member.read())
    PLISSKEN_BIN.chmod(0o755)
    return str(PLISSKEN_BIN)


def _render_api_docs():
    """Render the API reference pages with Plissken PLISSKEN_VERSION."""
    plissken = _plissken()
    if plissken is None:
        return 1

    print(f"Rendering the API docs with plissken {PLISSKEN_VERSION}...")
    # Remove the old pages first, so a module that is gone leaves no page.
    if API_ROOT.exists():
        shutil.rmtree(API_ROOT)
    result = subprocess.run([plissken, "render"], cwd=str(PROJECT_ROOT))
    if result.returncode != 0:
        print("ERROR: plissken render failed", file=sys.stderr)
        return result.returncode
    print(f"API docs written to {API_ROOT.relative_to(PROJECT_ROOT)}")
    return 0


@docs()
@angreal.command(
    name="api",
    about="Render the API docs only (plissken render)",
    tool=angreal.ToolDescription("""
Render the API reference pages and ``_sidebar.mjs`` into
``docs/src/content/docs/reference/api/`` with the pinned Plissken release.
On the first run it downloads that release into ``target/``. The pages are
not in git; ``build`` and ``serve`` render them too.

## When to use
- To look at the rendered API pages without building the site

## Examples
```
angreal docs api
```
""", risk_level="safe")
)
def api():
    """Render the API reference pages."""
    return _render_api_docs()


def _npm(*args):
    """Run npm in docs/, installing the dependencies on the first run."""
    npm = shutil.which("npm")
    if not npm:
        print("npm not found: install Node 22 or later", file=sys.stderr)
        return 1
    if not (DOCS_ROOT / "node_modules").exists():
        print("Installing documentation dependencies (npm ci)...")
        result = subprocess.run(
            [npm, "ci", "--no-audit", "--no-fund"], cwd=str(DOCS_ROOT)
        )
        if result.returncode != 0:
            return result.returncode
    return subprocess.run([npm, *args], cwd=str(DOCS_ROOT)).returncode


@docs()
@angreal.command(
    name="clean",
    about="Clean documentation build artifacts",
    tool=angreal.ToolDescription("""
Clean documentation build output: ``docs/dist`` and the Astro cache
``docs/.astro``. The rendered API pages stay; ``build`` renders them again.

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
    """Clean documentation build artifacts."""
    return _clean_docs()


@docs()
@angreal.command(
    name="serve",
    about="Start local documentation server with live reload",
    tool=angreal.ToolDescription("""
Render the API docs with Plissken, then serve the Starlight site at
http://localhost:4321 with live reload. In a terminal it runs until Ctrl+C.
From a script or an agent, Astro runs it in the background: stop it with
``npx astro dev stop`` in ``docs/``.

## When to use
- During documentation writing
- For previewing documentation changes
- When reviewing documentation locally

## When NOT to use
- In production environments
- For final documentation builds (use ``build``)

## Examples
```
angreal docs serve
```
""", risk_level="safe")
)
def serve():
    """Serve the documentation site locally with regenerated API docs."""
    print("=== Setting up documentation ===")

    # Regenerate the API reference first so the served site is current.
    api_result = _render_api_docs()
    if api_result != 0:
        return api_result

    print("\n=== Starting the Starlight dev server ===")
    return _npm("run", "dev")


@docs()
@angreal.command(
    name="build",
    about="Build production documentation site with generated API docs",
    tool=angreal.ToolDescription("""
Render the API docs with Plissken, then build the Starlight site into
``docs/dist``. Installs the npm dependencies on the first run. Fails when a
page does not build.

## When to use
- For production deployments
- Before releasing documentation
- For final documentation review

## When NOT to use
- During active documentation writing (use ``serve`` instead)
- For quick local previews

## Examples
```
angreal docs build
```
""", risk_level="safe")
)
def build():
    """Build the documentation site with regenerated API docs."""
    print("=== Building documentation site ===")

    # Clean previous artifacts for a reproducible build.
    clean_result = _clean_docs()
    if clean_result != 0:
        return clean_result

    # Regenerate the API reference.
    api_result = _render_api_docs()
    if api_result != 0:
        return api_result

    print("\nBuilding the Starlight site...")
    result = _npm("run", "build")
    if result == 0:
        print("\n=== Build complete ===")
        print(f"Documentation site built in {DOCS_ROOT / 'dist'}")
    return result
