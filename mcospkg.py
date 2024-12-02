import argparse
import os
from modules import pgb, color
import json

# Define args for input
parser = argparse.ArgumentParser(description='Install/Remove/Update/Download packages by using mcospkg')
parser.add_argument("options", type=str, help="Define Options for package(options are: install/remove/update/download)")
parser.add_argument("packages", type=str, nargs="*", help="Define the target package.")
parser.add_argument('-y', '--bypass', action='store_true', help="To bypass asking install/remove/update packages")


#print(args.options, args.packages, args.bypass)

# Set Essential Path:
CONFIG_DIR = 'etc/mcospkg'	# To build replace to '/etc/mcospkg'
CACHE_DIR = 'var/cache/mcospkg'	# To build replace to '/var/cache/mcospkg'

## Define optional options
def file_exist(filename):
    """This function checks is file exist.
    If yes, return True, otherwise return False."""
    
    try:
        f = open(filename, "r")
    except:
        return False
    else:
        f.close()
        return True

def check_is_repocfg_exist():
    """This check if repo configuation file exists.
    If yes, return True, else return False."""
    if file_exist(CONFIG_DIR + '/repo.conf'):
        return True
    return False

def preset():
    """This will prepare to run the program, automatically check the important things.
    If something wrong, it will return a int value(if return=0 it hasn't problems.)"""
    if check_is_repocfg_exist():	# The repocfg is the essential.
        repos = [] 	# This will get repo.conf status
        with open(CONFIG_DIR + '/repo.conf') as file:
            for line in file.readlines():
                parts = line.strip().split('=')
                repos.extend(parts)
        repos.pop(-1) 	# Delete the unusual thing
        """Now the list 'repos' has a regularity:
        	if subscript is evennum, this is the reponame
			Otherwise, it the repo url."""
		
        # Get repo name and repo url(split them)
        reponame = []
        repourl = []
        for subscript in range(0, len(repos) - 1, 2):
            reponame.append(repos[subscript])
        for subscript in range(len(repos)):
            if subscript % 2 != 0:
                repourl.append(repos[subscript])
        
        # Check if REPOINFO exists
        for repo in reponame:
            infofile = f"{CONFIG_DIR}/database/remote/{repo}.json"
            if not file_exist(infofile):
                return 2
           with open(infofile) as file:
               json.load(file)
            ...

# Define essential options
def download(packages):
    ...

# Load args
if __name__ == "__main__":
    args = parser.parse_args()
    preset()
