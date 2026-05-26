import os
import shutil
import subprocess
import tempfile

here = os.path.dirname(__file__)
functional_test_folder = os.path.join(here,"functional_tests")


def _make_in_place_template(root, *, roots=("{{ folder_variable }}",)):
    """Build a throwaway template under ``root``.

    Creates ``angreal.toml`` plus one or more top-level templated directories.
    Each templated directory contains a README, a nested ``src/`` file, and a
    ``.angreal/init.py`` so the rendered project is a valid angreal project.
    Returns the template directory path.
    """
    template = os.path.join(root, "template")
    os.makedirs(template, exist_ok=True)
    with open(os.path.join(template, "angreal.toml"), "w") as f:
        f.write('folder_variable = "folder_name"\n')
    for r in roots:
        inner = os.path.join(template, r)
        os.makedirs(os.path.join(inner, "src"), exist_ok=True)
        os.makedirs(os.path.join(inner, ".angreal"), exist_ok=True)
        with open(os.path.join(inner, "README.md"), "w") as f:
            f.write("# {{ folder_variable }}\n")
        with open(os.path.join(inner, "src", "main.txt"), "w") as f:
            f.write("hello\n")
        with open(os.path.join(inner, ".angreal", "init.py"), "w") as f:
            f.write("def init():\n    pass\n")
    return template


def test_init_in_place_strips_root():
    """`angreal init --in-place` renders contents into cwd, no root dir."""
    work = tempfile.mkdtemp()
    try:
        template = _make_in_place_template(work)
        dest = os.path.join(work, "dest")
        os.makedirs(dest)
        rv = subprocess.run(
            ["angreal", "init", template, "--in-place", "-d"],
            cwd=dest, capture_output=True, text=True,
        )
        assert rv.returncode == 0, f"stdout={rv.stdout}\nstderr={rv.stderr}"
        assert os.path.isfile(os.path.join(dest, "README.md"))
        assert os.path.isfile(os.path.join(dest, "src", "main.txt"))
        assert os.path.isdir(os.path.join(dest, ".angreal"))
        assert os.path.isfile(os.path.join(dest, ".angreal", "angreal.toml"))
        # the templated root directory was stripped, not created
        assert not os.path.exists(os.path.join(dest, "folder_name"))
    finally:
        shutil.rmtree(work, ignore_errors=True)


def test_init_in_place_errors_on_multiple_roots():
    """In-place with multiple top-level templated dirs aborts non-zero."""
    work = tempfile.mkdtemp()
    try:
        template = _make_in_place_template(
            work, roots=("{{ folder_variable }}", "{{ folder_variable }}_two"),
        )
        dest = os.path.join(work, "dest")
        os.makedirs(dest)
        rv = subprocess.run(
            ["angreal", "init", template, "--in-place", "-d"],
            cwd=dest, capture_output=True, text=True,
        )
        assert rv.returncode != 0
        # nothing rendered into cwd
        assert os.listdir(dest) == []
    finally:
        shutil.rmtree(work, ignore_errors=True)


def test_init_in_place_errors_on_zero_roots():
    """In-place with no top-level templated dir aborts non-zero."""
    work = tempfile.mkdtemp()
    try:
        template = os.path.join(work, "template")
        os.makedirs(os.path.join(template, "plain_dir"))
        with open(os.path.join(template, "angreal.toml"), "w") as f:
            f.write('folder_variable = "folder_name"\n')
        dest = os.path.join(work, "dest")
        os.makedirs(dest)
        rv = subprocess.run(
            ["angreal", "init", template, "--in-place", "-d"],
            cwd=dest, capture_output=True, text=True,
        )
        assert rv.returncode != 0
    finally:
        shutil.rmtree(work, ignore_errors=True)


def test_init_in_place_collision_requires_force():
    """In-place collisions abort without --force and overwrite with it."""
    work = tempfile.mkdtemp()
    try:
        template = _make_in_place_template(work)
        dest = os.path.join(work, "dest")
        os.makedirs(dest)
        # pre-existing colliding file
        with open(os.path.join(dest, "README.md"), "w") as f:
            f.write("OLD CONTENT")

        # without --force: abort, leave the existing file untouched
        rv = subprocess.run(
            ["angreal", "init", template, "--in-place", "-d"],
            cwd=dest, capture_output=True, text=True,
        )
        assert rv.returncode != 0
        with open(os.path.join(dest, "README.md")) as f:
            assert f.read() == "OLD CONTENT"

        # with --force: overwrite
        rv = subprocess.run(
            ["angreal", "init", template, "--in-place", "-d", "--force"],
            cwd=dest, capture_output=True, text=True,
        )
        assert rv.returncode == 0, f"stdout={rv.stdout}\nstderr={rv.stderr}"
        with open(os.path.join(dest, "README.md")) as f:
            assert "folder_name" in f.read()
    finally:
        shutil.rmtree(work, ignore_errors=True)


def test_group_1():
    """test a basic nested command with flag"""
    rv = subprocess.run([
        "angreal",
        "group1",
        "flag",
        "-t"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder,"group.txt"))
    except Exception as e:
        raise e
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder,"group.txt"))
        except Exception:
            pass
    pass


def test_group_2():
    """test a multi nested command with flag"""
    rv = subprocess.run([
        "angreal",
        "group1",
        "group2",
        "flag2",
        "-t"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder,"nested_group.txt"))
    except Exception as e:
        raise e
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder,"nested_group.txt"))
        except Exception:
            pass
    pass


def test_task_verbose_flag():
    """test that a task can define --verbose / -v flags"""
    # Test with --verbose
    rv = subprocess.run([
        "angreal",
        "verbose-test",
        "--verbose"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder, "verbose_test.txt"))
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder, "verbose_test.txt"))
        except Exception:
            pass

    # Test with -v short flag
    rv = subprocess.run([
        "angreal",
        "verbose-test",
        "-v"
    ], cwd=functional_test_folder)
    try:
        assert rv.returncode == 0
        assert os.path.exists(os.path.join(functional_test_folder, "verbose_test.txt"))
    finally:
        try:
            os.unlink(os.path.join(functional_test_folder, "verbose_test.txt"))
        except Exception:
            pass


def test_top_level_build():
    """test top-level 'build' runs with its own --release arg"""
    rv = subprocess.run(
        ["angreal", "build", "--release"],
        cwd=functional_test_folder, capture_output=True, text=True,
    )
    marker = os.path.join(
        functional_test_folder, "top_build_release.txt"
    )
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(marker)
    finally:
        try:
            os.unlink(marker)
        except Exception:
            pass


def test_docs_build():
    """test grouped 'docs build' runs with --format (not --release)"""
    rv = subprocess.run(
        ["angreal", "docs", "build", "--format", "pdf"],
        cwd=functional_test_folder, capture_output=True, text=True,
    )
    marker = os.path.join(
        functional_test_folder, "docs_build_pdf.txt"
    )
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(marker)
    finally:
        try:
            os.unlink(marker)
        except Exception:
            pass


def test_docs_build_no_cross_contamination():
    """test 'docs build' doesn't get --release from top-level 'build'"""
    # If args cross-contaminate, this will error with
    # "unexpected argument" or create the wrong marker file.
    rv = subprocess.run([
        "angreal", "docs", "build"
    ], cwd=functional_test_folder, capture_output=True, text=True)
    expected = os.path.join(
        functional_test_folder, "docs_build_html.txt"
    )
    unwanted = os.path.join(functional_test_folder, "top_build.txt")
    try:
        assert rv.returncode == 0, (
            f"stdout={rv.stdout}\nstderr={rv.stderr}"
        )
        assert os.path.exists(expected), (
            "docs build should create docs_build_html.txt"
        )
        assert not os.path.exists(unwanted), (
            "docs build should NOT trigger top-level build"
        )
    finally:
        for f in [expected, unwanted]:
            try:
                os.unlink(f)
            except Exception:
                pass
