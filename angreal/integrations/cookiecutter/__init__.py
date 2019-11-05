"""
    angreal.integrations.cookiecutter
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    Re-writing cookiecutter core functions to work with angreal syntax.
"""

from __future__ import unicode_literals
import os
import re
import json

from cookiecutter.config import get_user_config
from cookiecutter.generate import generate_context
from angreal.integrations.cookiecutter.prompt import prompt_for_config
from angreal.integrations.cookiecutter.generate import generate_files
from cookiecutter.utils import rmtree, make_sure_path_exists


from cookiecutter.exceptions import RepositoryNotFound
from cookiecutter.vcs import clone
from cookiecutter.zipfile import unzip

REPO_REGEX = re.compile(r"""(?x)
((((git|hg)\+)?(git|ssh|https?):(//)?)  # something like git:// ssh:// etc.
 |                                      # or
 (\w+@[\w\.]+)                          # something like user@...
)
""")


def is_repo_url(value):# pragma: no cover
    """Return True if value is a repository URL."""
    return bool(REPO_REGEX.match(value))


def is_zip_file(value):# pragma: no cover
    """Return True if value is a zip file."""
    return value.lower().endswith('.zip')


def expand_abbreviations(template, abbreviations):# pragma: no cover
    """Expand abbreviations in a template name.

    :param template: The project template name.
    :param abbreviations: Abbreviation definitions.
    """
    if template in abbreviations:
        return abbreviations[template]

    # Split on colon. If there is no colon, rest will be empty
    # and prefix will be the whole template
    prefix, sep, rest = template.partition(':')
    if prefix in abbreviations:
        return abbreviations[prefix].format(rest)

    return template


def repository_has_cookiecutter_json(repo_directory):# pragma: no cover
    """Determine if `repo_directory` contains a `cookiecutter.json` file.

    :param repo_directory: The candidate repository directory.
    :return: True if the `repo_directory` is valid, else False.
    """
    repo_directory_exists = os.path.isdir(repo_directory)

    repo_config_exists = os.path.isfile(
        os.path.join(repo_directory, 'angreal.json')
    )
    return repo_directory_exists and repo_config_exists


def determine_repo_dir(template, abbreviations, clone_to_dir, checkout,
                       no_input, password=None):# pragma: no cover
    """
    Locate the repository directory from a template reference.

    Applies repository abbreviations to the template reference.
    If the template refers to a repository URL, clone it.
    If the template is a path to a local repository, use it.

    :param template: A directory containing a project template directory,
        or a URL to a git repository.
    :param abbreviations: A dictionary of repository abbreviation
        definitions.
    :param clone_to_dir: The directory to clone the repository into.
    :param checkout: The branch, tag or commit ID to checkout after clone.
    :param no_input: Prompt the user at command line for manual configuration?
    :param password: The password to use when extracting the repository.
    :return: A tuple containing the cookiecutter template directory, and
        a boolean descriving whether that directory should be cleaned up
        after the template has been instantiated.
    :raises: `RepositoryNotFound` if a repository directory could not be found.
    """
    template = expand_abbreviations(template, abbreviations)

    if is_zip_file(template):
        unzipped_dir = unzip(
            zip_uri=template,
            is_url=is_repo_url(template),
            clone_to_dir=clone_to_dir,
            no_input=no_input,
            password=password
        )
        repository_candidates = [unzipped_dir]
        cleanup = True
    elif is_repo_url(template):
        cloned_repo = clone(
            repo_url=template,
            checkout=checkout,
            clone_to_dir=clone_to_dir,
            no_input=no_input,
        )
        repository_candidates = [cloned_repo]
        cleanup = False
    else:
        repository_candidates = [
            template,
            os.path.join(clone_to_dir, template)
        ]
        cleanup = False

    for repo_candidate in repository_candidates:
        if repository_has_cookiecutter_json(repo_candidate):
            return repo_candidate, cleanup

    raise RepositoryNotFound(
        'A valid repository for "{}" could not be found in the following '
        'locations:\n{}'.format(
            template,
            '\n'.join(repository_candidates)
        )
    )


def get_file_name(replay_dir, template_name):# pragma: no cover
    file_name = '{}.json'.format(template_name)
    return os.path.join(replay_dir, file_name)


def dump(replay_dir, template_name, context):# pragma: no cover
        """dump our replay to disk.

        [description]
        :param replay_dir: Where should the replay go ?
        :type replay_dir: str
        :param template_name: originating template name
        :type template_name: str
        :param context: the context applied to the template
        :type context: dict
        """

        if not make_sure_path_exists(replay_dir):
            raise IOError('Unable to create replay dir at {}'.format(replay_dir))

        if not isinstance(template_name, str):
            raise TypeError('Template name is required to be of type str')

        if not isinstance(context, dict):
            raise TypeError('Context is required to be of type dict')

        replay_file = get_file_name(replay_dir, template_name)

        with open(replay_file, 'w') as outfile:
            json.dump(context, outfile)


def cookiecutter(template, no_input=False, output_dir='.'):# pragma: no cover
    """A clone of cookiecutter's main entry point.

    This does alot less than cookiecutter's main entry point but flavors it for ``angreal`` .

    :param template: The path to the template to use.
    :type template: str
    :param no_input: Don't use the defaults, defaults to False
    :type no_input: bool, optional
    """

    config_dict = get_user_config(
        config_file=None,
        default_config=False)

    repo_dir, cleanup = determine_repo_dir(
        template=template,
        abbreviations=config_dict['abbreviations'],
        clone_to_dir=config_dict['cookiecutters_dir'],
        checkout=None,
        no_input=no_input,
        password=None
        )

    template_name = os.path.basename(os.path.abspath(repo_dir))

    context_file = os.path.join(repo_dir, 'angreal.json')

    context = generate_context(
                context_file=context_file,
                default_context=config_dict['default_context'],
                extra_context=None
                )

    context['angreal'] = prompt_for_config(context, no_input)
    context['angreal']['_template'] = template

    dump(config_dict['replay_dir'], template_name, context)

    result = generate_files(repo_dir=repo_dir,
                            context=context,
                            overwrite_if_exists=False,
                            output_dir=output_dir
                            )

    if cleanup:
        rmtree(cleanup)

    return result
