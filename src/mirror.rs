// First, import some modules we need
use clap::{Parser, Subcommand};
use colored::Colorize;
use mcospkg::download;
use mcospkg::readcfg;
use std::process::{exit, Command};
use std::io::Write;

// And then we define the arguments
#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = "0.1.1-debug")]
struct Args {
    #[command(subcommand)]
    option: Options,
}

#[derive(Subcommand, Debug)]
enum Options {
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
    match args.option {
        Options::Update => update(),
        Options::Add { reponame, repourl } => {
            add(reponame, repourl);
        },
        Options::Delete { reponame } => {
            delete(reponame);
        },
    }
}

fn update() {
    let error = "error".red().bold();

    // First, we read the configuration file
    let repoindex: Vec<(String, String)>;
    match readcfg() {
        Err(e) => {
            eprintln!("{}: {}", error, e);
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
        println!("Creating directory /etc/mcospkg/database/remote...");
        match Command::new("mkdir")
            .arg("-p")
            .arg("/etc/mcospkg/database/remote")
            .status()
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}: {}", error, e);
                exit(2);
            }
        }
    }

    // Third, download the file
    println!("Updating index file...");
    for ((reponame, repourl), msg) in repoindex.into_iter().zip(repo_msgs.into_iter()) {
        if let Err(errmsg) = download(
            format!("{}/PKGINDEX.json", repourl),
            format!("/etc/mcospkg/database/remote/{}.json", reponame),
            msg,
        ) {
            eprintln!("{}: {}", error, errmsg);
        }
    }
}

fn add(reponame: String, repourl: String) {
    let error = "error".red().bold();
    // First, open the repo file
    let repofile = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/mcospkg/repo.conf");
    match repofile {    // See if the file is opened
        Err(e) => { // If not, print the error and exit
            eprintln!("{}: {}", error, e);
            exit(2);
        },
        Ok(mut repofile) => {
            match repofile.write_all(format!("[{}]\nurl = {}\n", reponame, repourl).as_bytes()) {    // Write the file
                Err(e) => {     // If not, print the error and exit
                    eprintln!("{}: {}", error, e);
                    exit(2);
                },
                Ok(_) => {  // If yes, print the message
                    println!(
                        "{}: Added {} to the repository.",
                        reponame,
                        "ok".green().bold()
                    );
                }
            }
        }
    }
}

fn delete(_reponame: String) {}
