#!/bin/bash
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
PREFIX="/usr/local"	# You can specify another path

# Also, you need to install these dependencies:
# 	python(<=3.12, with pip) - Some module(s) written by using python;
# 	rust(<=2021 Edition, with cargo) - mainly using rust;
#
# Well, in different operating system(OS), python is different
# name, such as python/python3, so you need to define it:
PYTHON_NAME="python"	# Usually choose in "python" or "python3"

# pip is the same as python, such as pip/pip3, so we also define it:
PIP_NAME="pip" # Usually choose in "pip" or "pip3"

# So the configure is basically completed. Now, we'll start thr coding to build/install/remove the project.

# Step 1: install dependencies in python
# Now, we'll install dependencies in your python environment,
# so we'll install the packages via the requirements.txt.
echo "Step 1: Installing the python dependencies via requirements.txt ..."
echo "=====OUTPUT====="
$PIP_NAME install -r requirements.txt
echo "=====END OF OUTPUT====="
echo ""

# Next, we need to build the main rust code.
# That's very easy, just need to use cargo to build it.
# So, let's do:
echo "Step 2: Building project via cargo..."
echo "=====OUTPUT====="
cargo build --release
echo "=====END OF OUTPUT====="
echo ""

# Third, we need to intergrate it to the structure like /usr.
# Note: Building things are in target/release
#
echo "Step 3: Intergrating file structure..."
mkdir -p ./target/intergrated
cd target
find . -type f -name "mcospkg*"! -name "*.*" -print -exec cp {} "intergrated/" \;
