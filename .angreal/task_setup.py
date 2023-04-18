import angreal
import os

from angreal.integrations.venv import VirtualEnv

project_root = os.path.join(angreal.get_root(),'..')

@angreal.command(name="set-up", about="get a development environment setup")
def setup():
    venv = VirtualEnv(path=os.path.join(project_root,'.venv'),now=True,
                      requirements=['maturin','pre-commit','pytest'])
    venv.install_requirements()
