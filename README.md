# Mcospkg
Welcome to use mcospkg, a linux package manager by a 13-year-old boy

This project uses in MinecraftOS, a **linux** operating system for Minecraft players

## Description
This project imagine by a 12-year-old boy (THE SAME PERSON) at first, cause there's less package manager by Chinese, so as a Chinese young man and developer, I, and my team, will take this mission on.

## Build & Install
To build it, ensure you had installed these applications(packages):

 - python(>=3.12)
 - Rust(stable, latest, with Cargo)
 - gcc/clang
 - git

After installing it, follow these steps:

1, Clone from repository

Run this command:

```bash
git clone https://github.com/zhangxuan2011/mcospkg.git
cd mcospkg
```

2, Configure build.sh

Find the variable define prefix, python(pip) executable name, and modify the variable's value by your want

3, Build

Now, you just need to run these commands:
```bash
chmod +x build.sh
./build.sh
```
This will help you to build the mcospkg into the directory in `target/intergrated`.

4, Install

Run:
```bash
sudo ./build.sh install
```
This command will help you to install mcospkg into the prefix you've defined

5, Uninstall
If you don't want mcospkg in your computer, you just need to run:
```bash
sudo ./build.sh remove
```
**Attention!** You must build from this repository before you run it, otherwise we don't care about it if you get something worse!!!!!
