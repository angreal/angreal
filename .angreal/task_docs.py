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
"""

import subprocess
import sys
import shutil
from pathlib import Path

import angreal  # type: ignore

# Project root for accessing docs, etc. (one level up from .angreal)
PROJECT_ROOT = Path(angreal.get_root()).parent

# Define command group
docs = angreal.command_group(name="docs", about="commands for documentation tasks")


def _clean_docs():
    """Clean the documentation build directory."""
    public_dir = PROJECT_ROOT / "docs" / "public"
    if public_dir.exists():
        print("Cleaning documentation build directory...")
        shutil.rmtree(public_dir)
        print("Clean complete!")
    return 0


def _integrate_rustdoc():
    """Generate rustdoc and integrate it with the Hugo documentation site."""
    print("Generating rustdoc...")

    # Generate rustdoc
    try:
        subprocess.run(
            ["cargo", "doc", "--no-deps"],
            check=True
        )
    except subprocess.CalledProcessError as e:
        print(f"Failed to generate rustdoc: {e}", file=sys.stderr)
        return e.returncode

    # Setup paths
    hugo_docs_dir = PROJECT_ROOT / "docs"
    rustdoc_output_dir = PROJECT_ROOT / "target/doc"
    hugo_api_dir = hugo_docs_dir / "static/api"

    # Create Hugo API directory if it doesn't exist
    hugo_api_dir.mkdir(parents=True, exist_ok=True)

    # Copy rustdoc output to Hugo static directory
    print("Copying rustdoc output to Hugo...")
    try:
        # Use rsync for better file copying (preserves permissions,
        # handles existing files better)
        subprocess.run(
            ["rsync", "-av", "--delete", f"{rustdoc_output_dir}/", str(hugo_api_dir)],
            check=True
        )
    except subprocess.CalledProcessError as e:
        print(f"Failed to copy rustdoc output: {e}", file=sys.stderr)
        return e.returncode

    print("Rustdoc integration complete!")
    return 0


@docs()
@angreal.command(name="clean", about="clean the documentation build directory")
def clean():
    """Clean the documentation build directory."""
    return _clean_docs()


@docs()
@angreal.command(
    name="serve",
    about="serve the documentation site locally, by default building draft documents."
)
@angreal.argument(
    name="prod",
    long="prod",
    help="exclude draft content from the build",
    required=False,
    takes_value=False,
    is_flag=True
)
def serve(prod: bool = False):
    """Serve the Hugo documentation site locally with integrated API docs.

    Args:
        prod: If True, excludes draft content from the build. Defaults to False.
    """
    print("=== Setting up documentation ===")

    # Clean the build directory first
    clean_result = _clean_docs()
    if clean_result != 0:
        return clean_result

    # First integrate rustdoc
    print("\nIntegrating API documentation...")
    rustdoc_result = _integrate_rustdoc()
    if rustdoc_result != 0:
        return rustdoc_result

    # Then start Hugo server
    print("\n=== Starting Hugo server ===")
    print("Documentation will be available at http://localhost:1313")
    print("Press Ctrl+C to stop the server")

    try:
        # By default include drafts (-D), unless prod flag is set
        cmd = ["hugo", "server", "-D"]
        if prod:
            cmd.remove("-D")
            print("Excluding draft content from build")
        else:
            print("Including draft content in build")

        result = subprocess.run(
            cmd,
            cwd=str(PROJECT_ROOT / "docs"),
            check=True
        )
        return result.returncode
    except subprocess.CalledProcessError as e:
        print(f"Hugo server failed: {e}", file=sys.stderr)
        return e.returncode


@docs()
@angreal.command(
    name="build",
    about="build the documentation site, by default excluding draft documents."
)
@angreal.argument(
    name="draft",
    long="draft",
    help="include draft content in the build",
    required=False,
    takes_value=False,
    is_flag=True
)
def build(draft: bool = False):
    """Build the Hugo documentation site with integrated API docs.

    Args:
        draft: If True, includes draft content in the build. Defaults to False.
    """
    print("=== Building documentation site ===")

    # Clean the build directory first
    clean_result = _clean_docs()
    if clean_result != 0:
        return clean_result

    # First integrate rustdoc
    print("\nIntegrating API documentation...")
    rustdoc_result = _integrate_rustdoc()
    if rustdoc_result != 0:
        return rustdoc_result

    # Then build Hugo site
    print("\nBuilding Hugo site...")
    try:
        # By default exclude drafts, unless draft flag is set
        cmd = ["hugo"]
        if draft:
            cmd.append("-D")
            print("Including draft content in build")
        else:
            print("Excluding draft content from build (production mode)")

        result = subprocess.run(
            cmd,
            cwd=str(PROJECT_ROOT / "docs"),
            check=True
        )
        if result.returncode == 0:
            print("\n=== Build complete ===")
            print(f"Documentation site built in {PROJECT_ROOT}/docs/public")
        return result.returncode
    except subprocess.CalledProcessError as e:
        print(f"Failed to build documentation: {e}", file=sys.stderr)
        return e.returncode
