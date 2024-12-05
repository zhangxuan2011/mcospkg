import argparse
import os
from modules import pgb, color
from modules.file import check_file_exist as file_exist
import json
from sys import argv
import sys

# Define args for input
parser = argparse.ArgumentParser(description='Install/Remove/Update/Download packages by using mcospkg')
parser.add_argument("options", type=str, help="Define Options for package(options are: install/remove/update/download)")
parser.add_argument("packages", type=str, nargs="*", help="Define the target package.")
parser.add_argument('-y', '--bypass', action='store_true', help="To bypass asking install/remove/update packages")


#print(args.options, args.packages, args.bypass)

# Set Essential Path:
CONFIG_DIR = 'etc/mcospkg'	# To build replace to '/etc/mcospkg'
CACHE_DIR = 'var/cache/mcospkg'	# To build replace to '/var/cache/mcospkg'       

# Define essential options
def download(packages):
    ...

# Load args
if __name__ == "__main__":
    args = parser.parse_args()
    preset()
