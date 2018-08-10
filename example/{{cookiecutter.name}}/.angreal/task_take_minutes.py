import angreal

import datetime
import os
import subprocess
import tempfile


@angreal.command(name='take_minutes')
@angreal.option('--now',is_flag=True,help='start taking minutes immediately (requires EDITOR to be set)')
def angreal_cmd(now):
    """
    create a file for taking minutes
    """
    file_name = datetime.datetime.today().strftime('%Y-%m-%d-%H-%M')

    editor = os.environ.get('EDITOR',None)

    # Create our default file template using the current time as a header
    (fd, path) = tempfile.mkstemp()
    with open(fd, 'w') as default:
        print('# {}'.format(file_name), file=default)

    # We want to start writing now if we're able
    if now and editor:
        subprocess.call('{} {}'.format(editor,path), shell=True)


    # Send the finalized contents of the temporary file to the actual file
    with open(file_name+'.md', 'w') as dst:
        with open(path,'r') as src:
            print(src.read(),file=dst)

    # Clean up behind our selves
    os.unlink(path)


