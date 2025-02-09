/// This file contains the executable file `mcospkg`.
///
/// Usage:
///
/// ```bash
/// mcospkg [OPTIONS] <OPTION> <PACKAGES...>
/// ```
///
/// Explain:
///
/// [OPTIONS]: Including the `-y`, `-h` and `-V`;
///
/// <OPTION> : Be different with [OPTIONS], it's a string, only `install`, `remove` and `update`;
///
/// <PACKAGES> : The package you want to install/update/remove
///
/// This is the explaining of [OPTIONS]:
///
/// -y, --bypass: Install/remove/update packages WITHOUT asking;
/// -h, --help  : Get help message;
/// -V, --version: Print the version to the screen
///
/// Example:
///
/// ```bash
/// mcospkg install python  # Install package called "python"
/// mcospkg remove apt      # Remove the package called "apt"
/// mcospkg update          # Update all packages to the latest version
/// mcospkg update mcospkg  # Update the package "mcospkg" to the latest version
/// ```
///
/// For more information, type: `mcospkg -h`
//
// Now, we need to import some modules:
mod main {
    pub mod install;
}
use main::install;
use clap::Parser;
use dialoguer::Input;
use libc::c_char;
use mcospkg::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::process::exit;
use toml;

// Import C Libs(libpkgmgr.a)
extern "C" {
    fn removePackage(package_name: *const c_char);
}

// ========structs define area=========

// ====Arguments define area====
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.1-debug")]

// Define argument lists
struct Args {
    #[arg(
        required = true,
        help = "Supports: install/remove/remove-all/update/reinstall"
    )]
    option: String,

    #[arg(required = false)]
    packages: Vec<String>,

    #[arg(
        long = "bypass",
        short = 'y',
        default_value_t = false,
        help = "Specify it will not ask ANY questions"
    )]
    bypass_ask: bool,
}

// =====toml define area=====
// This defines the toml format (/etc/mcospkg/database/package_info.toml)
// This is for uninstall only
#[derive(Debug, Deserialize, Serialize)]
struct PkgInfoToml {
    dependencies: Vec<String>,
    version: String,
}

// ========functions define area==========
fn main() {
    let color = Color::new();
    let args = Args::parse();
    match args.option.as_str() {
        "install" => install(args.packages, args.bypass_ask, false),
        "remove" => remove(args.packages, args.bypass_ask),
        "reinstall" => install(args.packages, args.bypass_ask, true),
        _ => println!("{}: unknown option: {}", color.error, args.option),
    };
}

fn install(pkglist: Vec<String>, bypass_ask: bool, reinstall: bool) {
    // Presets
    let color = Color::new();

    // Tell user is this mode is "reinstall"
    if reinstall {
        println!("{}: Reinstall mode has been enabled.", color.note);
    }

    // Next, check if pkgindex is empty
    if pkglist.is_empty() {
        println!("{}: No package(s) specified.", color.error);
        exit(2);
    }

    // Init the InstallData struct
    let mut install = install::InstallData::new();

    // Stage 1: Get the pkgindex from the repositories
    print!("{}: Reading package index... ", color.info);
    install.step1();
    install.step2(pkglist.clone()); // Stage 2: Check if the package is exist
    println!("{}", color.done);

    // Stage 3: Check the packages' dependencies
    print!("{}: Checking package dependencies... ", color.info);
    install.step3(pkglist);
    println!("{}", color.done);

    // Stage 4: Download the package
    install.step4(reinstall);

    // Stage 5: Install the package
    install.step5(bypass_ask);

    // Stage 6: Install the package
    println!("{}: Installing package... ", color.info);
    install.step6();

    // And, that's complete!
}

// This is the install function
// =====S====P====L====I====T=====
// This is the remove function

fn remove(pkglist: Vec<String>, bypass_ask: bool) {
    // Presets
    let color = Color::new();

    // Ensure the pkglist is not empty
    if pkglist.is_empty() {
        eprintln!("{}: No package(s) specified.", color.error);
        exit(1);
    }

    // Stage 1: Explain the package
    // In "Remove" function, the most important is the dependencies.
    // In "/etc/mcospkg/package_info.toml", in each file's dependencies, defined it.
    // For example:
    /* [package_name]
       version = "0.1.1"
       dependencies = [
           "dep1",
           "dep2",
           "dep3",
           ...,
           "depn"
       ]
    */
    // Parse it
    let package_info_raw = std::fs::read_to_string("/etc/mcospkg/database/package_info.toml")
        .unwrap_or_else(|err| {
            // If it is not exist, quit
            eprintln!(
                "{}: Cannot read the package info \"/etc/mcospkg/database/package_info.toml\": {}",
                color.error, err
            );
            exit(1);
        });
    let package_info: HashMap<String, PkgInfoToml> = toml::from_str(&package_info_raw)
        .unwrap_or_else(|_| {
            eprintln!(
                "{}: Invaild format in \"/etc/mcospkg/database/package_info.toml\".",
                color.error
            );
            exit(1);
        }); // Main parsing code

    // Stage 2: Check the dependencies
    // Get its keys
    let mut package_info_keys: Vec<String> = Vec::new();
    for (key, _) in package_info.iter() {
        package_info_keys.push(key.clone());
    }

    // Make sure the specified the package is exist in that file
    // Check the HashMap's key is ok.
    let mut errtime = 0;
    for package in &pkglist {
        if !package_info_keys.contains(package) {
            eprintln!(
                "{}: Package \"{}\" is not installed, so we have no idea (T_T)",
                color.error, package
            );
            errtime += 1;
        }
    }

    if errtime > 0 {
        eprintln!("{}: {} errors occurred, terminated.", color.error, errtime);
        exit(1)
    }

    // Then let's see see...
    print!("{}: Resolving dependencies... ", color.info);

    // Read the vector "dependencies"
    let mut dependencies: Vec<String> = Vec::new();
    for pkg in &pkglist {
        for dep in &package_info[pkg].dependencies {
            dependencies.push(dep.clone());
        }
    }
    println!("{}", color.done);

    // Merge them
    let mut delete_pkgs = pkglist.clone();
    delete_pkgs.append(&mut dependencies);

    // Stage 3: Ask user
    println!("{}: The following packages will be removed:", color.info);
    for pkg in &delete_pkgs {
        print!("{} ", pkg);
    }
    println!(); // Make sure it can show normally

    if !bypass_ask {
        let input: String = Input::new()
            .with_prompt("\nDo you want to continue? (y/n)")
            .interact_text()
            .unwrap();
        if input != "y" && input != "Y" {
            println!("{}: User rejected the uninstallation request", color.error);
            exit(1);
        }
    } else {
        println!("\nADo you proceed to remove these packages? (y/n): y");
    }

    // Last, remove it.
    for delete_pkg in &delete_pkgs {
        let package_name = CString::new(delete_pkg.as_str()).unwrap();
        unsafe { removePackage(package_name.as_ptr()) }
    }
}
