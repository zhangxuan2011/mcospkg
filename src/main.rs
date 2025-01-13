// First, we should declare the lib
mod library {
    pub mod cfg;
}

// Now, we need to import some modules:
use crate::library::cfg::readcfg;
use clap::Parser;
use colored::Colorize;
use std::process::exit;
use std::path::Path;

// Configure parser
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.1-debug")]

// Define argument lists
struct Args {
    #[arg(required = true, help = "Supports: install/remove/update")]
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

fn main() {
    let error = "error".red().bold();
    let args = Args::parse();
    match args.option.as_str() {
        "install" => install(args.packages),
        "remove" => remove(args.packages),
        _ => println!("{}: unknown option: {}", error, args.option),
    };
}

fn install(pkgindex: Vec<String>) {
    let error = "error".red().bold();
    let tip = "tip".green().bold();
    // Stage 1: Explain the package
    // First, load configuration and get its HashMap
    let repoindex: Vec<(String, String)>;
    match readcfg() {
        Err(e) => {
            println!("{}: {}", error, e);
            println!(
                "{}: Consider using this format to write to that file:\n\t{}",
                "note".bold().green(),
                "[reponame] = [repourl]".cyan()
            );
            exit(2)
        },
        Ok(repoconf) => {
            repoindex = repoconf.into_iter().map(|(k, v)| (k,
v)).collect();
        }
    }

    // Second, check if index is exist
    let _repopath: String = String::new();  //  We'll use it later
    let mut errtime = 0;    // This will record the error times
    for (reponame, _) in repoindex {
        let repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        // If index not exist, just quit
        if! Path::new(&repopath).exists() {
            println!(
                "{}: Repository index \"{}\" not found",
                error, reponame
            );
            errtime += 1;
        }
    }
    if errtime > 0 {
        println!(
            "{}: use \"{}\" to download it.",
            tip,
            "mcospkg-mirror update".cyan()
        );
        exit(1);
    }

    // Next, check if pkgindex is empty
    if pkgindex.len() <= 0 {
        println!("{}: No package(s) specified.", error);
        exit(2);
    }

    // And, read the configuration
}

fn remove(pkgindex: Vec<String>) {}
