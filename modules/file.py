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