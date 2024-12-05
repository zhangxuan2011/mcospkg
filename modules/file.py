"""
DESCRIPTION
This uses in files options

REASON
Most of options needs to detect files,
According to the DRY principle, we have to make it as
a module.

USAGE
import by "from modules.file import *"

"""
def check_file_exist(filename):
    """This function checks is file exist.
    If yes, return True, otherwise return False."""
    
    try:
        f = open(filename, "r")
    except:
        return False
    else:
        f.close()
        return True
