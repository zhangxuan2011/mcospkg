# Imports
import urllib.request as fetch	# Fetch files 
import os	# To exec commands
import sys	# To get arguments
from modules import color	# To manage the program easily(this imports by folder 'modules').


# Get arguments
args = sys.argv

# Print help message that is len is only 1.
if len(args) == 1:
	print('Usage: mcospkg [options] packages \n'
	'\t[options]: the options to do for the packages.\n'
	'\tThe usual options are:\n\t'
	'\tinstall: To install packages;\n'
	'\t\tremove: To remove packages;\n'
	'\t\tupdate: To update packages.'
	'\n\tpackages: To defind a package for options.\n'
	'For example:\n'
	'\tmcospkg install base:\tTo install a package called base'
	'\n\tmcospkg remove apt:\tTo remove a package called apt\n'
	'To report bugs, welcome to send email to <zx20110412@outlook.com> or open an issue in <https://github.com/zhangxuan2011/mcospkg>. Have a nice day!\n')
elif len(args) == 2:	# It must lost options or pkg name.
	print(f'{color.red}error{color.end}: missing options or package name.\nEnter command without arguments to see help message.')
else:	# So this is available command
    options = args[1]	# Get options
    match options:
    	case 'install':
    		...