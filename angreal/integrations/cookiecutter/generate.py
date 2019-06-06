"""
    angreal.integrations.cookiecutter.generate
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Functions for generating a project from a project template.

    Used in place of cookiecutter.generate in order over ride some cookie cutter specific syntaxes.

"""
from __future__ import unicode_literals

import logging
import os
import shutil
from collections import OrderedDict

from jinja2 import FileSystemLoader
from jinja2.exceptions import UndefinedError

from cookiecutter.environment import StrictEnvironment
from cookiecutter.exceptions import UndefinedVariableInTemplate

from angreal.integrations.cookiecutter.find import find_template

from cookiecutter.utils import work_in, rmtree
from cookiecutter.generate import render_and_create_dir, ensure_dir_is_templated, generate_file, is_copy_only_path

logger = logging.getLogger(__name__)


def generate_files(repo_dir, context=None, output_dir='.',
                   overwrite_if_exists=False):
    """Render the templates and saves them to files.
   
    :param repo_dir: Project template input directory.
    :param context: Dict for populating the template's variables.
    :param output_dir: Where to output the generated project dir into.
    :param overwrite_if_exists: Overwrite the contents of the output directory  if it exists.
    """
    template_dir = find_template(repo_dir)
    logger.debug('Generating project from {}...'.format(template_dir))
    context = context or OrderedDict([])

    unrendered_dir = os.path.split(template_dir)[1]
    ensure_dir_is_templated(unrendered_dir)
    env = StrictEnvironment(
        context=context,
        keep_trailing_newline=True,
    )
    try:
        project_dir, output_directory_created = render_and_create_dir(
            unrendered_dir,
            context,
            output_dir,
            env,
            overwrite_if_exists
        )
    except UndefinedError as err:
        msg = "Unable to create project directory '{}'".format(unrendered_dir)
        raise UndefinedVariableInTemplate(msg, err, context)

    # We want the Jinja path and the OS paths to match. Consequently, we'll:
    #   + CD to the template folder
    #   + Set Jinja's path to '.'
    #
    #  In order to build our files to the correct folder(s), we'll use an
    # absolute path for the target folder (project_dir)

    project_dir = os.path.abspath(project_dir)
    logger.debug('Project directory is {}'.format(project_dir))

    # if we created the output directory, then it's ok to remove it
    # if rendering fails
    delete_project_on_failure = output_directory_created

    with work_in(template_dir):
        env.loader = FileSystemLoader('.')

        for root, dirs, files in os.walk('.'):
            # We must separate the two types of dirs into different lists.
            # The reason is that we don't want ``os.walk`` to go through the
            # unrendered directories, since they will just be copied.
            copy_dirs = []
            render_dirs = []

            for d in dirs:
                d_ = os.path.normpath(os.path.join(root, d))
                # We check the full path, because that's how it can be
                # specified in the ``_copy_without_render`` setting, but
                # we store just the dir name
                if is_copy_only_path(d_, context):
                    copy_dirs.append(d)
                else:
                    render_dirs.append(d)

            for copy_dir in copy_dirs:
                indir = os.path.normpath(os.path.join(root, copy_dir))
                outdir = os.path.normpath(os.path.join(project_dir, indir))
                logger.debug(
                    'Copying dir {} to {} without rendering'
                    ''.format(indir, outdir)
                )
                shutil.copytree(indir, outdir)

            # We mutate ``dirs``, because we only want to go through these dirs
            # recursively
            dirs[:] = render_dirs
            for d in dirs:
                unrendered_dir = os.path.join(project_dir, root, d)
                try:
                    render_and_create_dir(
                        unrendered_dir,
                        context,
                        output_dir,
                        env,
                        overwrite_if_exists
                    )
                except UndefinedError as err:
                    if delete_project_on_failure:
                        rmtree(project_dir)
                    _dir = os.path.relpath(unrendered_dir, output_dir)
                    msg = "Unable to create directory '{}'".format(_dir)
                    raise UndefinedVariableInTemplate(msg, err, context)

            for f in files:
                infile = os.path.normpath(os.path.join(root, f))
                if is_copy_only_path(infile, context):
                    outfile_tmpl = env.from_string(infile)
                    outfile_rendered = outfile_tmpl.render(**context)
                    outfile = os.path.join(project_dir, outfile_rendered)
                    logger.debug(
                        'Copying file {} to {} without rendering'
                        ''.format(infile, outfile)
                    )
                    shutil.copyfile(infile, outfile)
                    shutil.copymode(infile, outfile)
                    continue
                try:
                    generate_file(project_dir, infile, context, env)
                except UndefinedError as err:
                    if delete_project_on_failure:
                        rmtree(project_dir)
                    msg = "Unable to create file '{}'".format(infile)
                    raise UndefinedVariableInTemplate(msg, err, context)

    return project_dir
