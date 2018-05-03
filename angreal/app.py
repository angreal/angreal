import sys
import argparse
import inspect

from doit.doit_cmd import DoitMain
from angreal.utils import get_angreal_task_path,get_task_docs,get_task_names
from angreal.parser import get_init_parser,get_task_parser
from angreal.task_loader import AngrealTaskLoader


class AngrealApp(object):

    def __init__(self):
        """
        Initialize the application
        """

        self.tasks = set()
        #Do I look like I'm in an angreal project
        try:
            self.angreal_project = get_angreal_task_path()
        except FileNotFoundError:
            self.angreal_project = False


        if self.angreal_project:
            parser=get_task_parser()
            self.tasks=get_task_names()
        else :
            parser=get_init_parser()



        super_args = parser.parse_args(sys.argv[1:2])


        if hasattr(self,super_args.task):
            getattr(self,super_args.task)()
        elif super_args.task in self.tasks:
                self.doit()
        else:
            print('Un-recognized command "{}"'.format(super_args.task), file=sys.stderr)
            parser.print_help()
            sys.exit(1)

    def init(self):
        parser = get_init_parser()
        super_args = parser.parse_args(sys.argv[1:])
        print('initializing')

    def doit(self):
        exit(DoitMain(task_loader=AngrealTaskLoader()).run(sys.argv[1:]))



