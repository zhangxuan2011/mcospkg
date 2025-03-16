/// ## Information
/// Position: src/mirror/add.rs
/// Usage: The install library of src/mirror.rs
/// Date: 2025-03-16
/// Author: Xuan Zhang <zx20110412@outlook.com>
///
/// ## Description
/// This file is the library of src/mirror.rs.
/// It contains the struct `AddData` and its methods.
///
/// ## PS
/// The usage of this file is in src/main.rs.
/// Line 111-126 is the usage of this file.
/// (NOTE: The `src/main.rs` maybe update so that the lines may change.)
use mcospkg::Color;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

type FileResult = Result<std::fs::File, std::io::Error>;

pub struct AddData {
    repofile: FileResult,
}

impl AddData {
    pub fn new() -> Self {
        Self {
            repofile: OpenOptions::new()
                .write(true)
                .append(true)
                .open("/etc/mcospkg/repo.conf"),
        }
    }

    pub fn step_matches(
        &mut self,
        reponame: String,
        repourl: String,
    ) -> Result<(), Box<dyn Error>> {
        let color = Color::new();

        match &mut self.repofile {
            // See if the file is opened
            Err(e) => {
                // If not, print the color.error and exit
                eprintln!("{}: {}", color.error, e);
                exit(2);
            }
            &mut Ok(ref mut repo_file) => {
                match repo_file.write_all(format!("{} = {}\n", reponame, repourl).as_bytes()) {
                    // Write the file
                    Err(e) => {
                        // If not, print the color.error and exit
                        eprintln!("{}: {}", color.error, e);
                        exit(2);
                    }
                    Ok(_) => {
                        // If yes, print the message
                        println!(
                            "{}: Added repository name \"{}\" to the configuration file.",
                            color.ok,
                            reponame,
                        );
                        return Ok(())
                    }
                }
            }
        }
    }
}
