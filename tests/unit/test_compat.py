import os
import unittest

import angreal
from angreal.compat import get_template_version, is_compat
from tests import return_to_cwd


class CompatTests(unittest.TestCase):

    @return_to_cwd
    def test_get_template_versions(self):
        """ get version from template """
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        version = get_template_version()
        assert version == '>0.0.0'

        pass

    def test_is_compat(self):
        """ version is compat """
        os.chdir(os.path.join(os.path.dirname(__file__), 'fake-repo'))
        version = get_template_version()
        assert is_compat(version)
        assert not is_compat('>15.0.0')
        assert is_compat('=='+angreal.__version__)