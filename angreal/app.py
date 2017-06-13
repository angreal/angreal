# import argparse
# import os
# import shutil
# import sys
#
# import init
#
#
# class AngrealApp(object):
#     def __init__(self, args=sys.argv):
#         """
#         Main entry for the AngrealApp
#         :param args: sys.argv
#         """
#         super(AngrealApp, self).__init__()
#
#         parser = argparse.ArgumentParser(
#             description='Project Templating for Code Based Projects',
#             usage='''angreal <command> [<self.args>]
#
#             Commands are:
#             init      create a new angreal templated project
#             update      update a current angreal templated project
#             settings    access global and local settings for angreal
#             add_template    register a new template into angreal
#
#
#             For more specific information about a command use:
#             angreal <command> --help
#             ''',
#         )
#
#         self.args = args
#
#         parser.add_argument('command', help='a command to run')
#
#         super_args = parser.parse_args(self.args[1:2])
#
#         if not hasattr(self, super_args.command):
#             print('Un-recognized command {}'.format(self.args.command), file=sys.stderr)
#             parser.print_help()
#             sys.exit(1)
#
#         getattr(self, super_args.command)()
#
#     def init(self):
#         """
#         init sub command , initializes a angreal project
#         :return:
#         """
#         parser = argparse.ArgumentParser(
#             description='create an angreal project', usage='''
#             angreal init <project name> [optional arguments]
#
#             Arguments:
#             config      a specific configuration file to use
#             token       a personal token code to use for git-host support
#             ''')
#         parser.add_argument('project_name', help='The name for the project')
#         parser.add_argument(
#             '--config', help='use a specific complete configuration file')
#         parser.add_argument('--token', help='a personal token code to use')
#         init_args = parser.parse_args(self.args[2:])
#         init(init_args)
#         pass
#
#     def update(self):
#         """
#         update sub command, updates an angreal project - this behavior is still undefined.
#         :return:
#         """
#         parser = argparse.ArgumentParser(
#             description="update an angreal project", usage='angreal update')
#         update_args = parser.parse_args(self.args[2:])
#         pass
#
#     def config(self):
#         """
#         config sub command, lists current settings , or configures(and persists) a configuration parameter for angreal
#         """
#         parser = argparse.ArgumentParser(
#             description="access angreal config",
#             usage='''angreal config [<self.args>]
#             '''
#         )
#         parser.add_argument('--list', help='list current angreal settings')
#         parser.add_argument(
#             '--global', help='change a setting in the global config file (requires root)')
#         parser.add_argument(
#             '--local', help='change a setting in the local  config file')
#         config_args = parser.parse_args(self.args[2:])
#         pass
#
#     def add_template(self):
#         """
#         add_template: adds a template file to the angreal application.
#         :return:
#         """
#         parser = argparse.ArgumentParser(
#             description="register a template to angreal", usage='angreal register <file>')
#
#         parser.add_argument(
#             'file', nargs='+', help='the jinja2 template to register, must have a unique name')
#
#         self.args = parser.parse_args(self.args[2:])
#
#         for file in self.args.file:
#             src = os.path.abspath(file)
#             dst = os.path.join(os.path.abspath(os.path.dirname(
#                 __file__)), 'templates', os.path.basename(file.split('.')[0]))
#
#             if os.path.exists(dst):
#                 print("The template {} already exists, skipping".format(
#                     file), file=sys.stderr)
#             else:
#                 try:
#                     shutil.copy(src, dst)
#                 except OSError as e:
#                     print("{}".format(e), file=sys.stderr)
#                     pass
#                 except Exception as e:
#                     print("{}".format(e), file=sys.stderr)
#                     pass
#
#
# if __name__ == '__main__':
#     AngrealApp()
