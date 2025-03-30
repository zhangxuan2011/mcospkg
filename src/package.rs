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
mod config;
use clap::{Parser, Subcommand};
use config::VERSION;
use mcospkg::{Color, Package};
use mcospkg::{rust_install_pkg, rust_remove_pkg};

// Define args
#[derive(Parser, Debug)]
#[command(name = "mcospkg-package")]
#[command(about = "The lite installer of mcospkg.")]
#[command(version = VERSION)]
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

        #[arg(required = true, help = "The package path")]
        package_path: String,

        #[arg(required = true, help = "The version of the package")]
        package_version: String,
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
            // Make it to a struct
            let packages = Package::new(
                package_id,
                package_version,
                package_path,
            ).to_vec();

            // Finally use this function
            let status = rust_install_pkg(packages);
            if status != 0 {
                println!("{}: Installation failed with code: {}", color.error, status);
            }
        }
        Operations::Remove { package_id } => {
            let packages: Vec<String> = vec![package_id];
            let status = rust_remove_pkg(packages);
            if status != 0 {                                                      println!("{}: Uninstallation failed with code: {}", color.error, status);
            }
        }
    }
}
