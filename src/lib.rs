//! These functions is from "src/library" before. but there's only 1 function in each file.
//!
//! So I'll move them to here.

// Import the modules
mod pkgmgr;
use colored::{ColoredString, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{c_char, c_int, CStr};
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;
use std::process::exit;

// Public area
pub use pkgmgr::install_pkg as rust_install_pkg;
pub use pkgmgr::remove_pkg as rust_remove_pkg;

// =====toml define area=====
/// This defines the toml format (/etc/mcospkg/database/package.toml)
///
/// This is for uninstall only
#[derive(Debug, Deserialize, Serialize)]
pub struct PkgInfoToml {
    pub dependencies: Vec<String>,
    pub version: String,
}

/// In most codes, we usually meet the colorful message. So
/// we moved them in a struct.
pub struct Color {
    /// The color of the message
    /// This uses in lots of file, so we merged them here.
    pub error: ColoredString, // The error message
    pub tip: ColoredString,     // The tip message
    pub info: ColoredString,    // The info message
    pub done: ColoredString,    // The done message
    pub failed: ColoredString,  // The Failed Message
    pub note: ColoredString,    // The note message
    pub ok: ColoredString,      // The OK message
    pub no: ColoredString,      // The No message
    pub warning: ColoredString, // Thw warning message
}

// Implement this struct
impl Color {
    pub fn new() -> Self {
        Self {
            error: "error".red().bold(),
            tip: "tip".green().bold(),
            info: "info".blue().bold(),
            done: "Done".green().bold(),
            failed: "Failed".red().bold(),
            note: "note".yellow().bold(),
            ok: "OK".green().bold(),
            no: "No".red().bold(),
            warning: "warning".yellow().bold(),
        }
    }
}

// The error code defintions
pub enum ErrorCode {
    Skipped = 1, // For some option skipped
    Other = -1,  // Other error (more error code later)
}

fn convert_to_string(c_char_ptr: *const c_char) -> String {
    // Convert *const c_char to CStr first
    let c_str = unsafe { CStr::from_ptr(c_char_ptr) };
    // Convert CStr to String
    let result: String = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => String::new(),
    };
    result
}

// Export function for C
#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_install_pkg(
    package_id: *const c_char,
    package_path: *const c_char,
    version: *const c_char,
) -> c_int {
    // Convert the ptr to String first
    let package_id_rs = convert_to_string(package_id);
    let package_path_rs = convert_to_string(package_path);
    let version_rs = convert_to_string(version);

    // Make them to "Vec<Package>"
    // P.S: "Package" is a struct

    // To struct first
    let package = Package {
        id: package_id_rs,
        path: package_path_rs,
        version: version_rs,
    };

    // Then to the Vector
    let packages: Vec<Package> = vec![package];

    // Ask to the install function finally
    rust_install_pkg(packages)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_remove_pkg(package_name: *const c_char) -> c_int {
    // Convert to String first
    let package_name_rs = convert_to_string(package_name);

    // Append it to a vector
    let packages: Vec<String> = vec![package_name_rs];

    // Then use that function
    rust_remove_pkg(packages)
}

/// This structure defines the standard package info, and we will
/// install the package here.
///
/// # Field Explain
///
///  - id: The package ID name of the package (e.g. "python");
///  - path: The path of the package (e.g. /path/to/package.tar.xz);
///  - version: The version of the package (e.g. 0.1.0)
///
/// # Example
/// ```rust
/// let package = Package {
///     id: String::from("python"),
///     path: String::from("/path/to/python.tar.xz"),
///     version: String::from("3.12.8"),
/// };
/// // more options...
///
/// ```
#[derive(Debug)]
pub struct Package {
    pub id: String,
    pub path: String,
    pub version: String,
}

/// This function will read the configuration file and return a HashMap
///
/// The HaShMap's key is the repository name, and the value is the repository URL
///
/// If the configuration file is not found, it will return an error
///
/// If the configuration file is not in the correct format, it will return an error, too.
///
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
///
/// The URL is the URL of the file
///
/// The save is the path of the file to save
///
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
