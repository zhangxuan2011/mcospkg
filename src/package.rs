/// # Description
/// This file contains the file `mcospkg-package`, <br>
/// and it will install the package by using filename. <br>
/// <br>
/// # Usage
///
/// ```bash
/// mcospkg-package install [package_id] [package_version] [package_path]
/// mcospkg-package remove [package_id]
/// ```
///
/// # Explain
/// [package_id]: The ID of the package; <br>
/// [package_version]: The version of the package; <br>
/// [package_path]: The path of the package. <br>
///
/// # Example
/// ```bash
/// mcospkg-package install package 0.1.0 /path/to/package.tar.gz   # Install a package `package`, version `0.1.0`, in path`/path/to/package.tar.gz`
/// mcospkg-package remove package  # Remove the package `package`
/// ```
///
// Include some modules
use clap::{Parser, Subcommand};
use mcospkg::Color;
use mcospkg::{installPkg, removePkg};
use std::ffi::CString;

// Define args
#[derive(Parser, Debug)]
#[command(name = "mcospkg-package")]
#[command(about = "The lite installer of mcospkg.")]
#[command(version = "0.9.1 Build 9121")]
struct Args {
    #[command(subcommand)]
    operation: Operations,
}

#[derive(Subcommand, Debug)]
enum Operations {
    // The install option
    Install {
        #[arg(required = true, help = "The package ID")]
        package_id: String,

        #[arg(required = true, help = "The version of the package")]
        package_version: String,

        #[arg(required = true, help = "The package path")]
        package_path: String,
    },

    // The remove option
    Remove {
        #[arg(required = true, help = "The package ID you want to remove")]
        package_id: String,
    },
}

fn main() {
    // Presets
    let args = Args::parse();
    let color = Color::new();

    // Match it
    match args.operation {
        Operations::Install {
            package_id,
            package_version,
            package_path,
        } => {
            let package_path = CString::new(package_path).unwrap();
            let package_version = CString::new(package_version).unwrap();
            let package_id = CString::new(package_id).unwrap();
            let status = installPkg(
                package_path.as_ptr(),
                package_id.as_ptr(),
                package_version.as_ptr(),
            );
            if status != 0 {
                println!("{}: Installation failed with code: {}", color.error, status);
            }
        }
        Operations::Remove { package_id } => {
            removePkg(package_id.as_ptr());
        }
    }
}
