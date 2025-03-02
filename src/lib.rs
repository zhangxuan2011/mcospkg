// These functions is from "src/library" before. but there's only 1 function in each file.
// So I'll move them to here.

// Import the modules
use colored::{ColoredString, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use libc::{c_char, c_int};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;
use std::process::exit;

// =====toml define area=====
// This defines the toml format (/etc/mcospkg/database/package.toml)
// This is for uninstall only
#[derive(Debug, Deserialize, Serialize)]
pub struct PkgInfoToml {
    pub dependencies: Vec<String>,
    pub version: String,
}

// In most codes, we usually meet the colorful message. So
// we moved them in a struct.
pub struct Color {
    /// The color of the message
    /// This uses in lots of file, so we merged them here.
    pub error: ColoredString, // The error message
    pub tip: ColoredString,  // The tip message
    pub info: ColoredString, // The info message
    pub done: ColoredString, // The done message
    pub note: ColoredString, // The note message
    pub ok: ColoredString,   // The OK message
    pub no: ColoredString,   // The No message
}

// Implement this struct
impl Color {
    pub fn new() -> Self {
        Self {
            error: "error".red().bold(),
            tip: "tip".green().bold(),
            info: "info".blue().bold(),
            done: "Done".green().bold(),
            note: "note".yellow().bold(),
            ok: "OK".green().bold(),
            no: "No".red().bold(),
        }
    }
}

// The error code defintions
pub enum ErrorCode {
    Skipped = 1, // For some option skipped
    Other = -1 // Other error (more error code later)
}

// This will pub use the C function.
// Import it first
unsafe extern "C" {
    fn installPackage(
        package_path: *const c_char,
        package_name: *const c_char,
        version: *const c_char
    ) -> c_int;
    fn removePackage(
        package_name: *const c_char
    );
}

// Then export it
#[unsafe(no_mangle)]
pub extern "C" fn installPkg(
    package_path: *const c_char,
    package_name: *const c_char,
    version: *const c_char,
) -> c_int {
    unsafe {
        installPackage(package_path, package_name, version)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn removePkg(package_name: *const c_char) {
    unsafe {
        removePackage(package_name)
    }
}

/// This function will read the configuration file and return a HashMap
/// The HaShMap's key is the repository name, and the value is the repository URL
/// If the configuration file is not found, it will return an error
/// If the configuration file is not in the correct format, it will return an error, too.
/// The format is: `[reponame] = [repourl]`
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

/// This function will download a file from a URL and save it to a file
/// The URL is the URL of the file
/// The save is the path of the file to save
/// The msg is the message to show in the progress bar
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
            .template("{msg} {eta_precise} [{bar:40.green/blue}] {bytes}/{total_bytes} {binary_bytes_per_sec}\n")
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

pub fn get_installed_package_info() -> HashMap<String, PkgInfoToml> {
    let color = Color::new();
    let package_raw = std::fs::read_to_string("/etc/mcospkg/database/packages.toml")
        .unwrap_or_else(|_| {
            let file_path = Path::new("/etc/mcospkg/database/packages.toml");

            let mut _file = match File::create(&file_path) {
                Err(why) => {
                    println!(
                        "{}: couldn't create /etc/mcospkg/database/packages.toml: ({:?})",
                        color.error, why
                    );
                }
                Ok(_) => return Default::default(),
            };

            let return_value = "[null]\nversion=0\ndependencies = []\n\n".to_string();
            return_value
        });
    let package: HashMap<String, PkgInfoToml> = toml::from_str(&package_raw).unwrap_or_else(|_| {
        eprintln!(
            "{}: Invaild format in \"/etc/mcospkg/database/packages.toml\".",
            color.error
        );
        exit(1);
    }); // Main parsing code
    package
}
