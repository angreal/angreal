import angreal

import datetime
import os
import subprocess
import tempfile


@angreal.command(name="take-notes", about="Take notes for our meeting")
@angreal.argument(name="now", long="now", takes_value=False)
def angreal_cmd(now=False):
    """
    create a file for taking minutes
    """
    file_name = datetime.datetime.today().strftime("%Y-%m-%d-%H-%M")

    # We're going to assume that you're running on ubuntu
    # which has a binary called "editor" that will launch your
    # default terminal editor. If you need something else - set the environment
    # variable "EDITOR" to the appropriate command
    editor = os.environ.get("EDITOR", "editor")

    # Create our default file template using the current time as a header
    (fd, path) = tempfile.mkstemp()
    with open(fd, "w") as default:
        print("# {}".format(file_name), file=default)

    # We want to start writing now if we're able
    if now and editor:
        subprocess.call("{} {}".format(editor, path), shell=True)

    # Send the finalized contents of the temporary file to the actual file
    with open(file_name + ".md", "a") as dst:
        with open(path, "r") as src:
            print(src.read(), file=dst)

    # Clean up behind our selves
    os.unlink(path)
