import argparse
from modules import color, pgb
import sys

# Define Argument input
parser = argparse.ArgumentParser(prog='mcospkg-mirror.py', description="To manage mirror of mcospkg")

# Create subparsers
subparsers = parser.add_subparsers(dest='command', help='Sub-command help')

# Define the parser for the 'update' sub-command
update_parser = subparsers.add_parser('update', help='Perform update database operation')

# Define the parser for the 'add' sub-command
add_parser = subparsers.add_parser('add', help='Add repository operation')
add_parser.add_argument('reponame', type=str, help='Repository name')
add_parser.add_argument('repourl', type=str, help='Repository URL')

# Set Essential Path:
CONFIG_DIR = 'etc/mcospkg'  # To build replace to '/etc/mcospkg'  
CACHE_DIR = 'var/cache/mcospkg' # To build replace to '/var/cache/mcospkg'

# Define essential options
def add(name, url):
    new_repocfg = f"{name}={url}\n"
    with open(CONFIG_DIR + "/repo.conf", mode="a") as f: 
        f.writelines(new_repocfg)
    print(f"{color.green}ok{color.end}: repository \"{name}\" added successfully!")

if __name__ == "__main__":
    # Parse command line arguments
    args = parser.parse_args()
    # Check if no argument input
    match args.command:
        case None:
            print(f"{color.red}error{color.end}: no input argument\nuse argument \"-h\" for help.")
            sys.exit(2)
        case "add":
            add(args.reponame, args.repourl)

