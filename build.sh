#!/bin/sh
#
# This help you to build the mcosokg automatically without
# building by yourself.
#
# Now, I'm gotta say its usage of it.
# There are 2 option(s) you can choose:
# 	install: Help you to install mcospkg into your system.
# 	remove: Remove mcospkg
# For example:
# 	./build.sh - Just build the project and intergrate it;
# 	./build.sh install - Install it after building;
# 	./build.sh remove - Remove it from the prefix
# So, we need a prefix to do these, now we define it:
PREFIX="/"	# You can specify another path

# Also, you need to install these dependencies:
# 	python(<=3.12, with pip) - Some module(s) written by using python;
# 	rust(<=2021 Edition, with cargo) - mainly using rust, needs to compile;
#
# Well, in different operating system(OS), python is different
# name, such as python/python3, so you need to define it:
PYTHON_NAME="python"	# Usually choose in "python" or "python3"

# pip is the same as python, such as pip/pip3, so we also define it:
PIP_NAME="pip" # Usually choose in "pip" or "pip3"

# So the configure is basically completed. Now, we'll start thr coding to build/install/remove the project.
#
# We define the install/remove method:
remove() {
      local choice
      echo "Are you sure to remove mcospkg from your computer?(Enter Y or the other)"
      read choice
      if [ "$choice" != "Y" ] && [ "$choice" != "y" ]; then
            echo "Canceled the uninstallation"
            exit 1
      fi

      # So, let's start our remove works:
      rm $PREFIX/bin/mcospkg*
      rm -r $PREFIX/etc/mcospkg
      rm -r $PREFIX/var/cache/mcospkg
      echo "Ok: seems the works are completed."
      echo "See you next time!"
      exit 0
}
install() {
      echo "Moving project root to $PREFIX..."
      if [ ! -d target/intergrated ]; then
            echo "Error: Build file not found! Quiting..."
	    exit 1
      fi

      # Start to copy
      echo "=====OUTPUT====="
      cp -rv target/intergrated/bin/* $PREFIX/bin
      cp -rv target/intergrated/* $PREFIX
      echo "=====END OF OUTPUT====="
      echo "Installation completed."
      exit 0
}

# Well, we need to check that should we install/remove this 
# project to the linux system. We needs to input an argument
# to check it, so we do:
if [ "$1" = "install" ]; then
	echo "Performing install..."
	install
elif [ "$1" = "remove" ]; then
	remove
fi

# First, we need to build the main rust code.
# That's very easy, just need to use cargo to build it.
# So, let's do:
echo "Step 1: Building project via cargo..."
echo "=====OUTPUT====="
cargo build --release -j20
echo "=====END OF OUTPUT====="
echo ""

# Second, we need to intergrate it to the structure like /usr.
# Note: Building things are in target/release
echo "Step 2: Intergrating file structure..."
cd target	  # Do something in generated dirs
mkdir -p intergrated	# Make the dir to build the structure
mkdir -p intergrated/bin   # This uses in receiving bins
cd release	# So... you know
rm *.d	# First, we need to kill the .d file(They are fucking bad things)
cp mcospkg* ../intergrated/bin	# And, copy them
cd ../intergrated	# Finally, we have leave the dir
mkdir -p etc var	# Make structure continuly

# And, we needs to generate a configuration file (in etc)
mkdir -p etc/mcospkg
cd etc/mcospkg	# Enter and so something...
echo "main=https://zhangxuan2011.github.io/mcospkg/repo/main" > repo.conf	# Write configuration, you can also define a repo by yourself(This is default)
mkdir -p database   # Make the database dir
mkdir -p database/remote    # And, this will save the remote package info
mkdir -p database/locals    # This saves the locally installed packages information
cd ../..

# We've done the working in etc, now in var.
cd var
mkdir -p cache	# Cache only
mkdir -p cache/mcospkg
mkdir -p cache/mcospkg/pkgs
cd ../..

# And, that's completed!
echo "Done! project now in target/intergrated"
echo "Cleaning cache file(s)..."
rm -r release
echo "Done! Quiting..."

