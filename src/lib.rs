// These functions is from "src/library" before. but there's only 1 function in each file.
// So I'll move them to here.

// Import the modules
use colored::{ColoredString, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};

// In most codes, we usually meet the colorful message. So
// we moved them in a struct.
pub struct Color {
    pub error: ColoredString,
    pub tip: ColoredString,
    pub info: ColoredString,
    pub done: ColoredString,
    pub note: ColoredString,
}

// Implement this struct
impl Color {
    pub fn new() -> Self {
        let error = "error".red().bold();
        let tip = "tip".green().bold();
        let info = "info".blue().bold();
        let done = "Done".green().bold();
        let note = "note".yellow().bold();
        Self {
            error,
            tip,
            info,
            done,
            note,
        }
    }

    // This func will get the value of someone and return it.
    todo!()
}

// This function will read the configuration file and return a HashMap
// The HashMap's key is the repository name, and the value is the repository URL
// If the configuration file is not found, it will return an error
// If the configuration file is not in the correct format, it will return an error, too.
// The format is: [reponame] = [repourl]
pub fn readcfg() -> Result<HashMap<String, String>, Error> {
    // First, read the configuration
    let mut repoconf_raw = fs::read_to_string("/etc/mcospkg/repo.conf").map_err(|_| {
        Error::new(
            ErrorKind::Other,
            "Repository config file \"/etc/mcospkg/repo.conf\" not found",
        )
    })?;

    // Second, make it cleaner
    repoconf_raw = repoconf_raw.replace(" ", "").replace("\t", "");

    // Third, we remove the comments (with "#")
    repoconf_raw = repoconf_raw
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n");

    // Fourth, we convert it to the HashMap
    let mut repoconf: HashMap<String, String> = HashMap::new();
    for line in repoconf_raw.lines() {
        if let Some((key, value)) = line.split_once('=') {
            repoconf.insert(key.to_string(), value.to_string());
        }
    }

    // Finally, return it
    Ok(repoconf)
}

// This function will download a file from a URL and save it to a file
// The URL is the URL of the file
// The save is the path of the file to save
// The msg is the message to show in the progress bar
pub fn download(url: String, save: String, msg: &'static str) -> Result<(), Error> {
    let mut resp =
        get(url).map_err(|e| Error::new(ErrorKind::Other, format!("Cannot fetch file: {}", e)))?;
    let mut file = File::create(save).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!(
                "Cannot create file: {}. Perhaps you didn't run it with \"sudo\"?",
                e
            ),
        )
    })?;

    let pb = ProgressBar::new(resp.content_length().unwrap_or(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})\n")
            .unwrap()
            .progress_chars("##-"),
    );

    // Clone the msg string to get a 'static lifetime
    pb.set_message(msg);

    let mut downloaded_bytes = 0;
    let mut buffer = [0; 8192];
    loop {
        match resp.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                file.write_all(&buffer[0..bytes_read]).map_err(|e| {
                    Error::new(ErrorKind::Other, format!("Cannot write file: {}", e))
                })?;
                downloaded_bytes += bytes_read;
                pb.set_position(downloaded_bytes as u64);
            }
            _ => break,
        }
    }

    pb.finish();
    Ok(())
}
