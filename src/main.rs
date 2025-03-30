/// This file contains the executable file `mcospkg`.
///
/// # Usage
///
/// ```bash
/// mcospkg install [OPTION] <PACKAGES...>
/// mcospkg remove [OPTION] <PACKAGES...>
/// ```
///
/// # Explain
///
/// [OPTION]: Including the `-y`, `-h` and `-V`;
///
/// <PACKAGES> : The package you want to install/update/remove
///
/// This is the explaining of [OPTIONS]:
///
/// -y, --bypass: Install/remove/update packages WITHOUT asking;
/// -h, --help  : Get help message;
///
/// -V, --version: Print the version to the screen
///
/// # Example
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
mod config;
use clap::{Parser, Subcommand};
use config::VERSION;
use main::install;
use main::remove;
use mcospkg::Color;

// ========structs define area=========

// ====Arguments define area====
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = VERSION)]

// Define argument lists
struct Args {
    #[command(subcommand)]
    operation: Operations,
}

// Define Subcommand
#[derive(Subcommand, Debug)]
enum Operations {
    #[command(about = "Install the package(s).")]
    Install {
        // Get packages
        #[arg(required = true, help = "The package(s) you want to install")]
        packages: Vec<String>,

        // And should we bypass asking
        #[arg(
            long = "bypass",
            short = 'y',
            default_value_t = false,
            help = "Specify it will not ask ANY questions"
        )]
        bypass_ask: bool,

        // And should we reinstall it
        #[arg(
            long = "reinstall",
            short = 'r',
            default_value_t = false,
            help = "Specify it will (re)install package"
        )]
        reinstall: bool,
    },

    // And remove
    #[command(about = "Remove the package(s).")]
    Remove {
        // Get packages
        #[arg(required = true, help = "The package(s) you want to remove")]
        packages: Vec<String>,

        // And should we bypass asking
        #[arg(
            long = "bypass",
            short = 'y',
            default_value_t = false,
            help = "Specify it will not ask ANY questions"
        )]
        bypass_ask: bool,
    },
}

// ========functions define area==========
fn main() {
    let args = Args::parse();
    match args.operation {
        Operations::Install {
            packages,
            bypass_ask,
            reinstall,
        } => install(packages, bypass_ask, reinstall),
        Operations::Remove {
            packages,
            bypass_ask,
        } => remove(packages, bypass_ask),
    };
}

fn install(pkglist: Vec<String>, bypass_ask: bool, reinstall: bool) {
    // Presets
    let color = Color::new();

    // Tell user is this mode is "reinstall"
    if reinstall {
        println!("{}: Reinstall mode has been enabled.", color.note);
    }

    // Init the InstallData struct
    let mut install_data = install::InstallData::new();

    // Stage 1: Get the pkgindex from the repositories
    install_data.step1_explain_pkg(pkglist.clone()); // Stage 2: Check if the package is exist

    // Stage 2: Check the packages' dependencies
    install_data.step2_check_deps(pkglist.clone());

    // Stage 3: Check if package is installed
    install_data.step3_check_installed(reinstall);

    // Stage 4: Download the package
    install_data.step4_download(bypass_ask);

    // Stage 5: Check sha256sums integrity
    install_data.step5_check_sums();

    // Stage 6: Extract the package
    install_data.step6_extract();

    // Stage 7: Install the package
    install_data.step7_install();

    // And, that's complete!
}

// This is the install function
// =====S====P====L====I====T=====
// This is the remove function

fn remove(pkglist: Vec<String>, bypass_ask: bool) {
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
