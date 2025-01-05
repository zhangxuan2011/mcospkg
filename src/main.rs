// First, we should declare the lib
mod library {
    pub mod file;
    pub mod cfg;
}

// Now, we need to import some modules:
use clap::Parser;
use std::collections::HashMap;
use colored::Colorize;
use crate::library::cfg::readcfg;
use std::process::exit;

// Configure parser
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.0-debug")]

// Define argument lists
struct Args {
    #[arg(required = true, help = "Supports: install/remove/update")]
    options: String,
    
    #[arg(required = false)]
    packages: Vec<String>,

    #[arg(long = "bypass", short = 'y', default_value_t = false, help = "Specify it will not ask ANY questions")]
    bypass_ask: bool,
}

fn main() {
    let error = "error".red().bold();
    let args = Args::parse();
    match args.options.as_str() {
        "install" => install(args.packages),
        "remove" => remove(args.packages),
        _ => println!("{}: unknown option: {}", error, args.options),
    };    
}

fn install(pkgindex: Vec<String>) {
    let error = "error".red().bold();
    let tip = "tip".green().bold();
    // Stage 1: Explain the package
    // First, load configuration and get its HashMap
    let repoconf = readcfg();
    let repoindex = repoconf.keys();
    
    // Second, check if index is exist
    let mut repopath = String::new();
    let mut errtime = 0;
    for reponame in repoindex {
        repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        // If index not exist, just quit
        if! library::file::check_file_exist(&repopath) {
            println!("{}: Repository index \"{}\" not exist", error, reponame);
            errtime += 1;
        }
    }
    if errtime > 0 {
        println!("{}: use \"{}\" to download it.", tip, "mcospkg-mirror update".cyan());
        exit(1);
    }

    // Next, read the configuration

}

fn remove(pkgindex: Vec<String>) {}
