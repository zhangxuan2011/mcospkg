// First, import local modules
pub mod library {
    pub mod cfg;
    pub mod download;
}


// And import some modules we need
use clap::Parser;
use crate::library::cfg::readcfg;
use crate::library::download::download;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = "0.1.0-debug")]

struct Args {
    #[arg(required = true, help = "update/add/delete are the avainable option")]
    option: String,
}

fn main() {
    let args = Args::parse();
    match args.option.as_str() {
        "update" => update(),
        "add" => add(),
        "delete" => delete(),
        _ => todo!(),
    }
}

fn update() {
    // let error = "error".red().bold();
    // First, we read the configuration file
    let repoconf = readcfg();   // This in crate::library::cfg
    let repoindex = repoconf.keys();    // Get its key index

    // Second, download the file
    println!("Updating index file...");
    let repoindex: Vec<(String, String)> = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
    for (reponame, repourl) in repoindex {
        download(format!("{}/PKGINDEX.json", repourl), format!("/etc/mcospkg/database/remote/{}.json", reponame))
    }
}

fn add() {}

fn delete() {}
