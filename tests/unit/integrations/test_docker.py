import os
import sys

import pytest

in_cicd = os.environ.get('GITLAB_CI',False)

if not in_cicd:
    from angreal.integrations.docker import get_bound_host_ports, get_open_port, in_container, Container


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
    c.run('ls -la',verbose=False)

@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_container_in_container():
    """ we are in a container now"""
    c = Container('python:3')
    c.pull()
    mount_volume = os.path.join(os.path.dirname(__file__),'..','..','..')
    c.run('bash -c "cp -r /angreal angreal_test && cd angreal_test && pip install . && python -c \'from angreal.integrations.docker import in_container; assert in_container()\' "',
          volumes={mount_volume:{'bind': '/angreal', 'mode':'ro'}},verbose=False)

    result = c.container.wait()
    assert result['StatusCode'] == 0


@pytest.mark.skipif(in_cicd,reason="Doing these tests from within gitlab's CI is too hard")
def test_container_build():
    """we can  build and run"""
    with open("Dockerfile",'w') as f:
        print("""
FROM python:3
EXPOSE 5000
CMD ["python -c 'print("inside container")'"]
        """,file=f)

    c = Container('Dockerfile')
    c.build()
    c.run('')
    os.unlink('Dockerfile')