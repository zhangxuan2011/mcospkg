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
    pub mod remove;
}
use main::install;
use main::remove;
use clap::Parser;
use mcospkg::Color;
use std::process::exit;

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
    let mut install_data = install::InstallData::new();

    // Stage 1: Get the pkgindex from the repositories
    install_data.step1_explain_pkg(pkglist.clone()); // Stage 2: Check if the package is exist

    // Stage 3: Check the packages' dependencies
    install_data.step2_check_deps(pkglist.clone());

    // Stage 4: Download the package
    install_data.step3_check_installed(reinstall, pkglist.clone());

    // Stage 5: Install the package
    install_data.step4_download(bypass_ask);

    // Stage 6: Install the package
    install_data.step5_install();

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

    // Init the RemoveData struct
    let mut remove_data = remove::RemoveData::new();

    // Stage 1: Explain the package
    remove_data.step1_explain_pkg();

    // Stage 2: Check the dependencies
    remove_data.step2_check_deps(pkglist.clone());

    // Stage 3: Ask user
    remove_data.step3_ask_user(bypass_ask);

    // Stage 4: Remove the package
    remove_data.step4_remove();

    // Completed!!!!!!! :)
}
