from file import check_file_exist as file_exist

def split_repo_name_url():
    # Get repo name and repo url(split them)
    reponame = []
    repourl = []
    for subscript in range(0, len(repos) - 1, 2):
        reponame.append(repos[subscript])
    for subscript in range(len(repos)):
        if subscript % 2 != 0:
            repourl.append(repos[subscript])

def check_is_repocfg_exist(CONFIG_DIR):
    """This check if repo configuation file exists.
    If yes, return True, else return False."""
    if file_exist(CONFIG_DIR + '/repo.conf'):
        return True
    return False

def check_repo_conf_exist(CONFIG_DIR):
    """This will prepare to run the program, automatically check the important things.
    If something wrong, it will return a int value(if return=0 it hasn't problems.)"""
    if check_is_repocfg_exist():	# The repocfg is the essential.
        repos = [] 	# This will get repo.conf status
        with open(CONFIG_DIR + '/repo.conf') as file:
            for line in file.readlines():
                parts = line.strip().split('=')
                repos.extend(parts)
        if repos[-1] == '':
            repos.pop(-1) 	# Delete the unusual thing