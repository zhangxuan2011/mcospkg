#!/bin/sh
# This is the mcospkg installation script
# Be sure target/release exist
PREFIX="/"

# Check is build file exist
if [ ! -d target/release ]; then
    echo -e "Error: Build dir 'target/release' not found! Quiting..."
    exit 1
fi

echo "Step 1: Intergrating file structure..."
cd target	  # Do something in generated dirs
mkdir -p intergrated/bin   # This uses in receiving bins
cd release	# So... you know
rm *.d	# First, we need to kill the .d file(They are fucking bad things)
cp mcospkg* ../intergrated/bin	# And, copy them
cd ../intergrated	# Finally, we have leave the dir
mkdir -p etc var lib	# Make structure continuly

# And, we needs to generate a configuration file (in etc)
mkdir -p etc/mcospkg
cd etc/mcospkg	# Enter and so something...
echo "main = https://zhangxuan2011.github.io/mcospkg/repo/main" > repo.conf	# Write configuration, you can also define a repo by yourself(This is default)
mkdir -p database/remote    # And, this will save the remote package info
mkdir -p database/remove_info   # This saves the locally installed packages information
cd ../..

# We've done the working in etc, now in var.
cd var
mkdir -p cache/mcospkg
cd ../..

# Then in lib
cp lib/libpkgmgr.a intergrated/lib

# And, that's completed!
echo "Done! project now in target/intergrated"
echo "Cleaning cache file(s)..."
rm -r release
cd ..

# ====STEP 2====
echo "Step 2: Installation"
echo "Moving project root to $PREFIX..."
if [ ! -d target/intergrated ]; then
    echo "Error: Build file not found! Quiting..."
    exit 1
fi

# Start to copy
echo "=====OUTPUT====="
mkdir -p $PREFIX/bin
cp -rv target/intergrated/bin/* $PREFIX/bin
cp -rv target/intergrated/* $PREFIX
echo "=====END OF OUTPUT====="
echo "Installation completed."
exit 0