#!/bin/sh
# This is the mcospkg installation script
# Be sure target/release exist
PREFIX="/"

# The install function, in step 2.
install() {
    # Start to copy
    echo "=====OUTPUT====="
    sudo mkdir -p $PREFIX/bin
    sudo cp -rv target/intergrated/bin/* $PREFIX/bin
    sudo cp -rv target/intergrated/lib/* $PREFIX/lib
    sudo cp -rv target/intergrated/* $PREFIX
    echo "=====END OF OUTPUT====="
    echo "Installation completed."
}

# To intergrate something, in step 1
intergrate() {
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
touch database/packages.toml    # To save the installed package's info
    cd ../..

    # We've done the working in etc, now in var.
    cd var
    mkdir -p cache/mcospkg
    mkdir -p log/mcospkg
    cd ../..

# Then in lib
    cp release/libmcospkg.so intergrated/lib

# And, that's completed!
    echo "Done! project now in target/intergrated"
    echo "Cleaning cache file(s)..."
    rm -r release
    cd ..
}

# Check which option should we do in different condition.
if [ ! -d target/intergrated ]; then
    if [ ! -d target/release ]; then                                      echo "Error: The target/intergrated not exist!"
	echo "Please run \"cargo build --release\" to build it."
	exit 1
    fi
    # If the intergrated not exist, intergrate first
    echo "Note: You have not intergrate it yet, so we do it first"
    intergrate
fi

echo "Step 2: Installation"
echo "Moving project root to $PREFIX..."
install

exit 0
