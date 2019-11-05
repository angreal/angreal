from angreal.integrations.gitremote import GitRemote,repo_required

import pytest


def test_all():
    """
    basic tests against the git remote base class
    """
    gr = GitRemote()
    gr.remote = 1
    gr.repo = 1


    with pytest.raises(ValueError):
        gr.remote=2

    with pytest.raises(ValueError):
        gr.repo=2

    with pytest.raises(NotImplementedError):
        gr.get_repo(7)


    with pytest.raises(NotImplementedError):
        gr.create_repository('noop')

    with pytest.raises(NotImplementedError):
        gr.protect_branch('name')

    with pytest.raises(NotImplementedError):
        gr.add_label('test','red')

    with pytest.raises(NotImplementedError):
        gr.add_milestone('thing')

    with pytest.raises(NotImplementedError):
        gr.enable_issues()

    with pytest.raises(NotImplementedError):
        gr.enable_gitlfs()

    with pytest.raises(NotImplementedError):
        gr.destroy_project()


def test_decorator():
    """
    testing our repo_required decorator
    """
    class GR(GitRemote):

        @repo_required
        def test_repo(self):
            return



    gr = GR()

    with pytest.raises(ValueError):
        gr.test_repo()

    gr.repo = 2

    gr.test_repo()

    with pytest.raises(ValueError):
        class Test(object):
            def __init__(self):
                return
            @repo_required
            def noop():
                return

        t = Test()
        t.noop()