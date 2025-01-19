// First, import local modules
pub mod library {
    pub mod cfg;
    pub mod download;
}

// And import some modules we need
use crate::library::cfg::readcfg;
use crate::library::download::download;
use clap::Parser;
use colored::Colorize;
use std::process::exit;

// And then we define the arguments
#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = "0.1.1-debug")]

struct Args {
    #[arg(required = true, help = "update/add/delete are the avainable option")]
    option: String,
}

fn main() {
    let error = "error".red().bold();
    let args = Args::parse();
    match args.option.as_str() {
        "update" => update(),
        "add" => add(),
        "delete" => delete(),
        _ => println!("{}: unknown option: {}", error, args.option),
    }
}

fn update() {
    let error = "error".red().bold();

    // First, we read the configuration file
    let repoindex: Vec<(String, String)>;
    match readcfg() {
        Err(e) => {
            println!("{}: {}", error, e);
            println!(
                "{}: Consider using this format to write to that file:\n\t{}",
                "note".bold().green(),
                "[reponame] = [repourl]".cyan()
            );
            exit(2);
        }
        Ok(repoconf) => {
            repoindex = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
        }
    }

    // Fill the repo_msgs
    let mut repo_msgs: Vec<&'static str> = Vec::new();
    for (reponame, _) in repoindex.clone() {
        let msg = format!("{}", reponame);
        let msg = Box::leak(msg.into_boxed_str());
        repo_msgs.push(msg);
    }

    // Second, download the file
    println!("Updating index file...");
    for ((reponame, repourl), msg) in repoindex.into_iter().zip(repo_msgs.into_iter()) {
        if let Err(errmsg) = download(
            format!("{}/PKGINDEX.json", repourl),
            format!("/etc/mcospkg/database/remote/{}.json", reponame),
            msg,
        ) {
            println!("{}: {}", error, errmsg);
        }
    }
}

fn add() {}

fn delete() {}
