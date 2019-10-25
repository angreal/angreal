import os
from angreal.integrations.docker import get_bound_host_ports, get_open_port, in_container, Container
import pytest

in_cicd = os.environ.get('GITLAB_CI',False) == '1'

@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_bound_ports():
    """
    just test bound ports, in a clean environment this should be empty
    """

    assert not get_bound_host_ports()

@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_get_open_port():
    """
    test for an open port
    """
    assert isinstance(get_open_port(),int)

@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_not_in_container():
    """
    we are not in a container right now
    """

    assert not in_container()
@pytest.mark.skipif(in_cicd ,reason="Doing these tests from within gitlab's CI is too hard")
def test_containter_pull_run():
    """we can pull and run"""
    c = Container('python:3')
    c.pull()
    c.run('ls -la')

@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_container_in_container():
    """ we are in a container now"""
    c = Container('python:3')
    c.pull()
    mount_volume = os.path.join(os.path.dirname(__file__),'..','..','..')
    c.run('bash -c "cp -r /angreal angreal_test && cd angreal_test && pip install . && python -c \'from angreal.integrations.docker import in_container; assert in_container()\' "',volumes={mount_volume:{'bind': '/angreal', 'mode':'ro'}})

    result = c.container.wait()
    assert result['StatusCode'] == 0
