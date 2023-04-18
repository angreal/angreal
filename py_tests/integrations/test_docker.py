import pytest
import sys

from angreal.integrations.docker import Docker
from angreal.integrations.docker.container import Containers, Container
from angreal.integrations.docker.network   import Network, Networks
from angreal.integrations.docker.image     import Image, Images
from angreal.integrations.docker.volume    import Volume, Volumes


@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
@pytest.mark.skipif(
    sys.platform == 'darwin', reason="default osx file doesn't exist tests are flaky"
)
def test_client_init():
    """ client has expected methods&attrs"""
    d = Docker()
    assert isinstance(d,Docker)

    interface_methods = [
        "containers",
        "images",
        "networks",
        "volumes"
    ]

    for m in interface_methods:
        assert hasattr(d,m) and callable(getattr(d,m))

    gettr_methods = [
        "version",
        "info",
        "ping",
        "data_usage"
    ]

    for gm in gettr_methods:
        assert isinstance(getattr(d,gm)(),dict)

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_containers():
    """containers is a containers instance"""
    d = Docker()
    assert isinstance(d.containers(), Containers)
    assert isinstance(Container(d,'test'), Container)

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_volumes():
    """volumes interface exists"""
    d = Docker()
    assert isinstance(d.volumes(), Volumes)
    assert isinstance(Volume(d,'id'), Volume)

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_images():
    """images interface exists"""
    d = Docker()
    assert isinstance(d.images(), Images)
    assert isinstance(Image(d,'id'), Image)

@pytest.mark.skipif(
    sys.platform == 'win32', reason="windows tests are flaky"
)
def test_network():
    """volumes interface exists"""
    d = Docker()
    assert isinstance(d.networks(), Networks)
    assert isinstance(Network(d,'id'), Network)
