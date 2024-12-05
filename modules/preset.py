"""
DESCRIPTION
This uses in mcospkg stage "preset", as a module

REASON
Because it needs to use in most of files.
To obey "DRY" principle, we have to merge it in a code
By the way, cause it needs lots of variable "CONFIG_DIR", so we 
write it as a class.

USAGE
import it by using "from modules.preset import Preset" in the rootof directory,
and the function "Preset" need to input 1 argument: CONFIG_DIR
and... you know :)

NOTE
Run it straightly will got exception!!!
"""

# Import essential modules
from modules.file import check_file_exist as file_exist
from sys import argv
from modules import color
import json
import os

class Preset:
    def __init__(self, CONFIG_DIR):
        # Initialize objects
        self.CONFIG_DIR = CONFIG_DIR
        self.repos = [] 	# This will get repo.conf status
        self.reponame = []
        self.repourl = []
    
    def check_is_repocfg_exist(self):
        """This check if repo configuation file exists.
        If yes, return True, else return False."""
        if file_exist(self.CONFIG_DIR + '/repo.conf'):
            return True
        return False

    def split_repo_name_url(self):
        # Get repo name and repo url(split them)
        for subscript in range(0, len(self.repos) - 1, 2):
            self.reponame.append(self.repos[subscript])
        for subscript in range(len(self.repos)):
            if subscript % 2 != 0:
                self.repourl.append(self.repos[subscript])

    def check_repo_conf_exist(self):
        """This will prepare to run the program, automatically check the important things.
        If something wrong, it will return a int value(if return=0 it hasn't problems.)"""
        if self.check_is_repocfg_exist():	# The repocfg is the essential.
            with open(self.CONFIG_DIR + '/repo.conf') as file:
                for line in file.readlines():
                    parts = line.strip().split('=')
                    self.repos.extend(parts)
            if self.repos[-1] == '':
                self.repos.pop(-1) 	# Delete the unusual thing

    def check_if_repoinfo_exist(self):
        if not os.path.exists(f"{self.CONFIG_DIR}/database/remote"): # Ensure remote path is exist
            os.mkdir(f"{self.CONFIG_DIR}/database")
            os.mkdir(f"{self.CONFIG_DIR}/database/remote")
        
        for repo in self.reponame:
            infofile = f"{CONFIG_DIR}/database/remote/{repo}.json"
            if not file_exist(infofile):
                print(f"{argv[0]}: {color.red}error{color.end}: self.repository index \"{repo}\" not found\nUse \"mcospkg-mirror update\" to download.")
                sys.exit(2)
            with open(infofile) as file:
                json.load(file)
                print(file.read())
