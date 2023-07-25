import angreal
import os
import subprocess
import webbrowser

venv_path = os.path.join(angreal.get_root(),'..','.venv')

cwd = os.path.join(angreal.get_root(),'..')
docs_dir = os.path.join(cwd,"docs")

docs = angreal.command_group(name="docs", about="commands for generating documentation")

# @docs()
# @angreal.command(name="python")
# def build_python_api():
#     need to build a better way of reflecting the python api into documentation as markdown
#     that is automatable and aesthetically pleasing
#     pass

@docs()
@angreal.command(name="stop", about="stop the currently running hugo server")
def build_hugo():
    subprocess.run(["pkill -f hugo"], shell=True)

@docs()
@angreal.command(name="serve", about="starts the docs site in the background.")
def build_hugo():
    pid = subprocess.Popen(
        [
            "hugo serve -p 12345&",
        ], cwd=docs_dir, shell=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE
    ).pid

    print(pid)
    print(test)
    webbrowser.open_new("http://localhost:12345/angreal/")

    print(f"hugo server running on pid {pid}, to stop run kill -9 {pid}")
    print("or you can run `angreal docs stop`")
    return
