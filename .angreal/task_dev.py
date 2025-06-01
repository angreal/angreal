import angreal
import os
import subprocess
import shutil


project_root = os.path.join(angreal.get_root(),'..')

dev = angreal.command_group(name="dev", about="tasks for"
                            "the management of the dev experience")

def is_program_available(program_name):
    return shutil.which(program_name) is not None


@dev()
@angreal.command(name="install", about="install and "
                 "verify the development environment")
def setup():

    subprocess.run("python3 -m pip install -e .",
                   shell=True, cwd=project_root, check=True)
    # Setup the virtual environment as .venv in the root folder

    # Install pre commit
    subprocess.run("pre-commit install", shell=True, cwd=project_root)


    # Check for system level dependencies and flash
    # a message if they're not installed
    # We're not going to automate setup cause that's
    # more work than i'm interested in doing
    dependencies_required = (
        ("hugo" , "please visit : https://gohugo.io/installation/"),
        ("cargo", "curl --proto '=https' --tlsv1.2"
         " -sSf https://sh.rustup.rs | sh && rustup update")
    )

    missing_deps = True
    for dep in dependencies_required :
        if not is_program_available(dep[0]):
            print(f"{dep[0]} is not available install via {dep[0]}")
            missing_deps = True

    if missing_deps:
        print("You're missing some system level dependencies,"
              " please use the above instructions to install them.")
    return
