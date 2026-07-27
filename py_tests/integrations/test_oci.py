"""Tests for the angreal.integrations.oci binding.

Import/surface tests always run. The live round-trip is gated on
ANGREAL_OCI_TEST_REGISTRY (e.g. ``localhost:5555``); spin up a registry with::

    docker run -d --rm -p 5555:5000 registry:2
    ANGREAL_OCI_TEST_REGISTRY=localhost:5555 angreal test python
"""
import os

import pytest

from angreal.integrations.oci import Oci

REGISTRY = os.environ.get("ANGREAL_OCI_TEST_REGISTRY", "").strip()
requires_registry = pytest.mark.skipif(
    not REGISTRY, reason="set ANGREAL_OCI_TEST_REGISTRY to run live OCI tests"
)


def test_oci_importable_and_constructs():
    """The Oci class imports and instantiates."""
    client = Oci()
    assert client is not None


def test_oci_has_expected_methods():
    """The minimal surface is present."""
    client = Oci()
    for method in ("pull", "push", "tags", "login"):
        assert hasattr(client, method), f"Oci is missing {method}()"


def test_login_accepts_credentials():
    """login() stores credentials without contacting a registry."""
    client = Oci()
    # Should not raise; no network happens until a pull/push/tags call.
    client.login("ghcr.io", "user", "token")


def test_pull_invalid_reference_raises():
    """A malformed reference surfaces as a Python exception, not a crash."""
    client = Oci()
    with pytest.raises(Exception):
        client.pull("not a valid reference !!")


def _make_template(root):
    """Create a minimal valid angreal template under ``root``; return its path."""
    template = os.path.join(root, "template")
    inner = os.path.join(template, "{{ folder_variable }}")
    os.makedirs(os.path.join(inner, ".angreal"))
    with open(os.path.join(template, "angreal.toml"), "w") as f:
        f.write('folder_variable = "oci_py_rendered"\n')
    with open(os.path.join(inner, "README.rst"), "w") as f:
        f.write("# {{ folder_variable }}\n")
    with open(os.path.join(inner, ".angreal", "init.py"), "w") as f:
        f.write("def init():\n    pass\n")
    return template


@requires_registry
def test_push_pull_tags_roundtrip(tmp_path):
    """push -> tags -> pull round-trips through a live registry."""
    client = Oci()
    template = _make_template(str(tmp_path))
    reference = f"oci://{REGISTRY}/angreal-py-test/roundtrip:v1"

    client.push(reference, template)

    tags = client.tags(f"oci://{REGISTRY}/angreal-py-test/roundtrip")
    assert "v1" in tags

    dest = tmp_path / "pulled"
    out = client.pull(reference, str(dest))
    assert os.path.isdir(out)
    assert os.path.isfile(os.path.join(out, "angreal.toml"))
    assert os.path.isfile(
        os.path.join(out, "{{ folder_variable }}", "README.rst")
    )


@requires_registry
def test_pull_defaults_dest_to_repo_basename(tmp_path, monkeypatch):
    """pull() with no dest lands in a cwd dir named after the repo."""
    client = Oci()
    template = _make_template(str(tmp_path))
    reference = f"oci://{REGISTRY}/angreal-py-test/defaultdest:v1"
    client.push(reference, template)

    workdir = tmp_path / "work"
    workdir.mkdir()
    monkeypatch.chdir(workdir)

    out = client.pull(reference)
    assert os.path.basename(out) == "defaultdest"
    assert os.path.isfile(os.path.join(out, "angreal.toml"))
