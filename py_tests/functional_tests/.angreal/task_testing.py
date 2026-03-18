import angreal



group1 = angreal.command_group(name="group1", about="testing group")


@group1()
@angreal.command(name="flag", about="test a flag argument")
@angreal.argument(name="test", long="test", short="t", takes_value=False, is_flag=True)
def flag_tests(test):
    """
    """
    if test:
        open("group.txt","w").close()


@group1()
@angreal.group(name="group2", about="group2")
@angreal.command(name="flag2", about="test a flag argument")
@angreal.argument(name="test", long="test", short="t", takes_value=False, is_flag=True)
def flag_test2(test):
    """
    """
    if test:
        open("nested_group.txt","w").close()


@angreal.command(name="verbose-test", about="test task can use --verbose flag")
@angreal.argument(
    name="verbose", long="verbose", short="v", takes_value=False, is_flag=True
)
def verbose_test(verbose):
    """Test that tasks can define their own --verbose flag."""
    if verbose:
        open("verbose_test.txt", "w").close()


# --- Command name collision tests ---
# A top-level "build" and a grouped "docs build" share the same base name.
# Without unique registry keys the second registration silently overwrites the first,
# causing argument cross-contamination (e.g. `angreal docs build` receives `--release`).

docs_group = angreal.command_group(name="docs", about="documentation commands")


@angreal.command(name="build", about="compile the project")
@angreal.argument(
    name="release", long="release", short="r",
    takes_value=False, is_flag=True,
)
def top_level_build(release):
    """Top-level build command with --release flag."""
    if release:
        open("top_build_release.txt", "w").close()
    else:
        open("top_build.txt", "w").close()


@docs_group()
@angreal.command(name="build", about="build the docs")
@angreal.argument(
    name="format", long="format", short="f",
    takes_value=True, default_value="html",
)
def docs_build(format):
    """Grouped docs build command with --format arg."""
    open(f"docs_build_{format}.txt", "w").close()
