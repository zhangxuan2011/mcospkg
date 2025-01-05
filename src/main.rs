// First, we should declare the lib
mod library {
    pub mod file;
}

// Now, we need to import some modules:
use clap::Parser;
use std::collections::HashMap;
use colored::Colorize;
use std::fs;
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
    let error = "error".red();
    let args = Args::parse();
    match args.options.as_str() {
        "install" => install(args.packages),
        "remove" => remove(args.packages),
        _ => println!("{}: unknown option: {}", error, args.options),
    };    
}

fn install(pkgindex: Vec<String>) {
    let error = "error".red();
    // Stage 1: Explain the package
    //// First, we read the configuration file
    let mut repoconf_raw = fs::read_to_string("/etc/mcospkg/repo.conf").expect("Failed to read /etc/mcospkg/repo.conf, check is it exists");
    
    //// Second, make it cleaner
    repoconf_raw = repoconf_raw
        .replace(" ", "")
        .replace("\t", "");

    //// Third, we convert it to the HashMap
    let mut repoconf: HashMap<&str, &str> = HashMap::new();
    for line in repoconf_raw.lines() {
        if let Some((key, value)) = line.split_once('=') {
            repoconf.insert(key, value);
        }
    }
    let repoindex = repoconf.keys();
    
    //// Fourth, check if index is exist
    let mut repopath = String::new();
    for reponame in repoindex {
        repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        if! library::file::check_file_exist(&repopath) {
            println!("{}: Repository index {} not exist\n\tTip: use \"mcospkg-mirror update\" to download it.", error, reponame);
            exit(1);
        }
    }


}

fn remove(pkgindex: Vec<String>) {}
