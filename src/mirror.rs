// First, import some modules we need
mod config;
use config::VERSION;
use clap::{Parser, Subcommand};
use colored::Colorize;
use mcospkg::{download, readcfg, Color};
use std::process::{exit, Command};
use std::io::Write;

// And then we define the arguments
#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = VERSION)]
struct Args {
    #[command(subcommand)]
    operation: Operations,
}

#[derive(Subcommand, Debug)]
enum Operations {
    #[command(about = "Update the mirror list")]
    Update,

    #[command(about = "Add a mirror")]
    Add {
        #[arg(help = "The name of the repository")]
        reponame: String,

        #[arg(help = "The url of the repository")]
        repourl: String,
    },

    #[command(about = "Delete a mirror")]
    Delete {
        #[arg(help = "The name of the repository")]
        reponame: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.operation {
        Operations::Update => update(),
        Operations::Add { reponame, repourl } => {
            add(reponame, repourl);
        },
        Operations::Delete { reponame } => {
            delete(reponame);
        },
    }
}

fn update() {
    let color = Color::new();

    // First, we read the configuration file
    let repoindex: Vec<(String, String)>;
    match readcfg() {
        Err(e) => {
            eprintln!("{}: {}", color.error, e);
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

    // Second, create the dir if not exist
    // Dir we store database: /etc/mcospkg/database/remote
    if !std::path::Path::new("/etc/mcospkg/database/remote").exists() {
        println!("{}: Creating directory /etc/mcospkg/database/remote...", color.info);
        match Command::new("mkdir")
            .arg("-p")
            .arg("/etc/mcospkg/database/remote")
            .status()
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}: {}", color.error, e);
                exit(2);
            }
        }
    }

    // Third, download the file
    println!("{}: Updating index file...", color.info);
    for ((reponame, repourl), msg) in repoindex.into_iter().zip(repo_msgs.into_iter()) {
        if let Err(errmsg) = download(
            format!("{}/PKGINDEX.json", repourl),
            format!("/etc/mcospkg/database/remote/{}.json", reponame),
            msg,
        ) {
            eprintln!("{}: {}", color.error, errmsg);
        }
    }
}

fn add(reponame: String, repourl: String) {
    let color = Color::new();

    // First, open the repo file
    let repofile = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/mcospkg/repo.conf");
    match repofile {    // See if the file is opened
        Err(e) => { // If not, print the color.error and exit
            eprintln!("{}: {}", color.error, e);
            exit(2);
        },
        Ok(mut repofile) => {
            match repofile.write_all(format!("{} = {}\n", reponame, repourl).as_bytes()) {    // Write the file
                Err(e) => {     // If not, print the color.error and exit
                    eprintln!("{}: {}", color.error, e);
                    exit(2);
                },
                Ok(_) => {  // If yes, print the message
                    println!(
                        "{}: Added repository name \"{}\" to the configuration file.",
                        "ok".green().bold(),
                        reponame,
                    );
                }
            }
        }
    }
}

fn delete(_reponame: String) {}
