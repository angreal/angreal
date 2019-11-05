"""
    angreal.integrations.docker
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~

    integrations to docker
"""

import docker

import os

CLIENT = docker.from_env()



def in_container():
    """
    determine if we're running within a container or not
    :return: bool
    """

    def text_in_file( text, file):# pragma: no cover
        try:
            return any(text in line for line in open(file))
        except FileNotFoundError:
            return False


    return ( os.path.exists('./.dockerenv') or
            text_in_file('docker', '/proc/self/cgroup') )



class Container(object):
    """
    Describes a container and provides some minimal interactions with the container
    """

    def __init__(self, source):
        """
        :param source: the file or image name to use
        """

        self.source = source
        self.image = None
        self.client = docker.client.from_env()






    def pull(self):
        """
        pull an image from a registry
        :return:
        """
        self.image = self.client.images.pull(self.source)

    def build(self, context='.', tag=None, buildargs=None,verbose=True):
        """
        build a docker image from a file

        :param context: build context
        :param tag: tag for the image
        :param buildargs: any other build arguments to pass as a dictionary
        :return:
        """

        self.image, log = self.client.images.build(dockerfile = self.source,
                                                   path=context,
                                                   tag = tag,
                                                   nocache=True,
                                                   quiet=False,
                                                   pull=True,
                                                   forcerm=True,
                                                   buildargs=buildargs
                                                   )
        if verbose:
            for f in log:
                f = str(f.get('stream', '')).strip()
                if f:
                    print(f)

    def run(self,command, *args, detach=True, auto_remove=True, ports=None, volumes=None, verbose=True, **kwargs):
        """
        run your compiled image

        :param command: what command to run
        :param args: any other args that could be passed to docker.client.containers.run
        :param detach: should the container detach default = True
        :param auto_remove: should the container auto_remove default = True
        :param ports: dictionary of port bindings { container : host }
        :param volumes: dictionary of volume bindings { host : {'bind': conainer_path, 'mode': 'rw'}}
        :param kwargs: any otther kwargs that could be passed tot docker.client.containers.run

        """

        if not ports:
            ports = {}
        if not volumes:
            volumes = {}
        self.container = self.client.containers.run(self.image.id,
                                   command,
                                   *args,
                                   detach=detach,
                                   auto_remove=auto_remove,
                                   ports = ports,
                                   volumes = volumes,
                                   **kwargs)

        if verbose:
            for f in (self.container.logs(stream=True)):
                print(f.decode().strip())


def get_bound_host_ports():
    """
    Get a list of currently bound host ports

    :return:
    """

    ports_in_use = set()

    for c in CLIENT.containers.list():
        port_bindings = c.attrs.get('NetworkSettings', {}).get('Ports', {})

        for internal,external in port_bindings.items():
            for e in external:
                ports_in_use.add(e.get('HostPort',None))


    return ports_in_use


def get_open_port():
    """
    Get a potentially open port

    There is no reservation until the container run is called. As a result it is possible, however unlikely, that two
    containers would try to grab the same port.

    """
    possible_range = set([ str(x) for x in range(8000,9001) ]) ^ get_bound_host_ports()
    return  int(possible_range.pop())
