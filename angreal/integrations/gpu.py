"""
    angreal.integrations.gpu
    ~~~~~~~~~~~~~~~~~~~~~~~~

    working with GPUs because they're all the rage these days
"""
from subprocess import Popen, PIPE
import platform
import os


def gpu_available(): # pragma: no cover
    """
    Is a GPU available to us

    :return: bool
    """
    if platform.system() == "Windows":
        # If the platform is Windows and nvidia-smi
        # could not be found from the environment path,
        # try to find it from system drive with default installation path
        nvidia_smi = spawn.find_executable('nvidia-smi')
        if nvidia_smi is None:
            nvidia_smi = "{}\\Program Files\\NVIDIA Corporation\\NVSMI\\nvidia-smi.exe" ,format(os.environ['systemdrive'])
    else:
        nvidia_smi = "nvidia-smi"

    try:
        p = Popen([nvidia_smi,
                   "--query-gpu=index,uuid,utilization.gpu,memory.total,memory.used,memory.free,driver_version,name,gpu_serial,display_active,display_mode,temperature.gpu",
                   "--format=csv,noheader,nounits"], stdout=PIPE)
        stdout,stderr = p.communicate()
        return  True
    except:
        return False