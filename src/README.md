## Producers

XiaoKuai <rainyhowcool@outlook.com>
zhangxuan2011 <zx20110412@outlook.com>

## Description

This directory contains the source code

Why create this? Let me explain it:

## Structures

`main/` (directory): The library of `main.rs`;

`main.rs`: The main file (This exports the binary file `mcospkg`);

`lib.rs`: The library contains the thing for all the binary;

`info.rs`: Exports the binary `mcospkg-info`;

`mirror/` (directory): The library of `mirror.rs`;

`mirror.rs`: Exports the binary `mcospkg-mirror`;

`pkgmgr/` (directory): The install & remove library.

## Build

To build this project, you just need to run:

```bash
cargo build --release -j8   # Build this project.
sudo ../install.sh  # Install it, in "src" directory.
```
