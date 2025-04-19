/// ## Information
/// Position: src/mirror/update.rs
/// Usage: The update library of src/mirror.rs
/// Date: 2025-03-16
/// Author: Xuan Zhang <zx20110412@outlook.com>
///
/// ## Description
/// This file is the library of src/mirror.rs.
/// It contains the struct `UpdateData` and its methods.
///
/// ## PS
/// The usage of this file is in src/main.rs.
/// Line 111-126 is the usage of this file.
/// (NOTE: The `src/main.rs` maybe update so that the lines may change.)
use colored::Colorize;
use mcospkg::{Color, download, readcfg};
use std::process::{Command, exit};

// The type-alias
type Message = std::borrow::Cow<'static, str>;

// Define the public data
pub struct UpdateData {
    repoindex: Vec<(String, String)>,
    repo_msgs: Vec<Message>,
}

// Then add some public method
impl UpdateData {
    // Initialize function
    pub fn new() -> Self {
        Self {
            repoindex: vec![],
            repo_msgs: vec![],
        }
    }

    pub fn step1_readcfg(&mut self) {
        let color = Color::new();

        // First, we read the configuration file;
        match readcfg() {
            Err(e) => {
                eprintln!("{}: {}", color.error, e);
                eprintln!(
                    "{}: Consider using this format to write to that file:\n\t{}",
                    "note".bold().green(),
                    "[reponame] = [repourl]".cyan()
                );
                exit(2);
            }
            Ok(repoconf) => {
                self.repoindex = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
            }
        }
    }

    pub fn step2_fill_msgs(&mut self) {
        // Fill the repo_msgs
        for (reponame, _) in self.repoindex.as_slice() {
            let msg: Message = reponame.clone().into();
            self.repo_msgs.push(msg);
        }
    }

    pub fn step3_create_dirs(&self) {
        let color = Color::new();

        // Third, create the dir if not exist
        // Dir we store database: /etc/mcospkg/database/remote
        if !std::path::Path::new("/etc/mcospkg/database/remote").exists() {
            println!(
                "{}: Creating directory /etc/mcospkg/database/remote...",
                color.info
            );
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
    }

    pub fn step4_download_index(&mut self) {
        let color = Color::new();

        // Fourth, download the file
        println!("{}: Updating index file...", color.info);
        for ((reponame, repourl), msg) in self
            .repoindex
            .as_slice()
            .into_iter()
            .zip((&self.repo_msgs).into_iter())
        {
            if let Err(errmsg) = download(
                &format!("{}/PKGINDEX.json", repourl),
                &format!("/etc/mcospkg/database/remote/{}.json", reponame),
                msg.clone(),
            ) {
                eprintln!("{}: {}", color.error, errmsg);
            }
        }
    }
}
