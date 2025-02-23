# Mcospkg
Welcome to use mcospkg, a linux package manager by a 13-year-old boy

This project uses in MinecraftOS, a **linux** operating system for Minecraft players

## Description
This project imagine by a 12-year-old boy (THE SAME PERSON) at first, cause there's less package manager by Chinese, so as a Chinese young man and developer, I, and my team, will take this mission on.

## Extension Documents
For more documents, please look at the directory `Doc/`

如果你是中国人🇨🇳, 那么我们贴心地准备了翻译(在`Doc/Chinese`下)

## Build & Install
If you have installed mcospkg, just run this command:
`sudo mcospkg update mcospkg`

To build it, ensure you had installed these applications(packages):

 - Rust(stable, latest, with Cargo)
 - gcc/clang(with cc)
 - openssl & libssl-dev
 - pkg-config
 - git

After installing it, follow these steps:

1, Clone from repository

Run these commands:

```bash
git clone https://github.com/zhangxuan2011/mcospkg.git
cd mcospkg
```

2, Build
Run this command to build this project:

`cargo build --release -j8`

In it, you can specify the building jobs (In this example, Jobs = 8)

**NOTE**!!!! You must specify the argument `--release` otherwise you **CAN'T** do more steps in this building process. 

3, Install

Run the `install.sh` we've given:

`./install.sh`

This will install the mcospkg to `/` (Defined in `PREFIX`)
