import argparse
import os
import shutil
import sys

from angreal import log
from angreal import commands



class AngrealApp(object):
    def __init__(self, args=sys.argv):
        """
        Main entry for the AngrealApp
        :param args: sys.argv
        """
        super(AngrealApp, self).__init__()
        
        
        parser = argparse.ArgumentParser(
            description='Project Templating for Code Based Projects',
            usage='''angreal <command> [<self.args>]

            Commands are:
            init      create a new angreal templated project
            update      update a current angreal templated project
            settings    access global and local settings for angreal
            register    register a new template into angreal
            
            
            For more specific information about a command use:
            angreal <command> --help
            ''',
        )

        self.args = args

        parser.add_argument('command', help='a command to run')
        super_args = parser.parse_args(self.args[1:2])

        if not hasattr(self, super_args.command):
            print('Un-recognized command {}'.format(self.args.command), file=sys.stderr)
            parser.print_help()
            sys.exit(1)

        getattr(self, super_args.command)()

    def init(self):
        """
        init sub command , initializes a angreal project
        
        currently doesn't take any arguments, in the future, it is likely that individual config files and some
        enviroment variables will be able to be passed to the function
        :return:
        """
        parser = argparse.ArgumentParser(
            description='create an angreal project', usage='''
            angreal init [optional arguments]
            ''')
        args = parser.parse_args(self.args[2:])
        commands.init(args)

    def update(self):
        """
        update sub command, updates an angreal project - this behavior is still undefined.
        :return:
        """
        parser = argparse.ArgumentParser(
            description="update an angreal project", usage='angreal update')
        args = parser.parse_args(self.args[2:])
        commands.update(args)
        pass

    def config(self):
        """
        config sub command, lists current settings , or configures(and persists) a configuration parameter for angreal
        
        .. todo:
        Parameter setting still not supported. Especially in the instance of "global" variables, care will be needed to
        ensure that users have a hard time overriding global config settings.
        """
        parser = argparse.ArgumentParser(
            description="access angreal config",
            usage='''angreal config [<self.args>]
            '''
        )
        parser.add_argument('--list', help='list current angreal settings')
        parser.add_argument(
            '--global', help='change a setting in the global config file (requires root)')
        parser.add_argument(
            '--local', help='change a setting in the local  config file')
        args = parser.parse_args(self.args[2:])
        commands.config(args)

    def register(self):
        """
        registers a template to angreal
        :return:
        """
        parser = argparse.ArgumentParser(
            description="register a template to angreal", usage='angreal register <file>')

        parser.add_argument(
            'file', nargs='+', help='the jinja2 template to register, must have a unique name')

        args = parser.parse_args(self.args[2:])
        commands.register(args)




def main():
    """
    Main entry point for the angreal app
    :return: 
    """
    log.AngrealLogger().run()
    AngrealApp()
