//! These functions is from "src/library" before. but there's only 1 function in each file.
//!
//! So I'll move them to here.

// Import the modules
mod pkgmgr;
use colored::{ColoredString, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_int};
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use tar::Archive;
use xz2::read::XzDecoder;

// Type annotions area
pub type Message = std::borrow::Cow<'static, str>;

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

/// The error code defintions,
#[derive(Debug)]
pub enum ErrorCode {
    FileNotFound = 1,
    CopyFilesError = 2,
    PermissionDenied = 3,
    CleanDirError = 4,
    ChangeDirError = 5,
    CreateDirError = 6,
    ExecuteError = 7,
    Other = -1,
    // More error codes...
}

impl From<ErrorCode> for c_int {
    fn from(error: ErrorCode) -> c_int {
        error as c_int
    }
}

/// Convert c_char to String.
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
pub extern "C" fn c_install_pkg(
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
    let packages = Package::new(
        package_id_rs.clone(),
        package_path_rs.clone(),
        version_rs.clone(),
    )
    .to_vec();

    // And extract them
    let workdir = extract(&package_path_rs).unwrap();
    let workdirs = vec![workdir];

    // Ask to the install function finally
    match rust_install_pkg(packages, workdirs) {
        Ok(_) => 0,
        Err(error) => error.into(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn c_remove_pkg(package_name: *const c_char) -> c_int {
    // Convert to String first
    let package_name_rs = convert_to_string(package_name);

    // Append it to a vector
    let packages: Vec<String> = vec![package_name_rs];

    // Then use that function
    match rust_remove_pkg(packages) {
        Ok(_) => 0,
        Err(error) => error.into(),
    }
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
/// use mcospkg::Package;
///
/// let package = Package {
///     id: String::from("python"),
///     path: String::from("/path/to/python.tar.xz"),
///     version: String::from("3.12.8"),
/// };
/// // more options...
///
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub id: String,
    pub path: String,
    pub version: String,
}

// Implement it
impl Package {
    /// The initializer of this struct
    pub fn new(id: String, path: String, version: String) -> Self {
        Self { id, path, version }
    }

    /// This will help you to convert this struct to a vector.
    ///
    /// NOTE: Make sure it is mutable if you want to change
    /// its value.
    pub fn to_vec(&self) -> Vec<Self> {
        let vector = vec![self.clone()];
        vector
    }

    /// If you have 3 vectors, and they are all corresponding
    /// to each other one by one, you can use this to convert
    /// them to one vector.
    ///
    /// NOTE: It returns a vector, please iterate it if you
    /// only want one!!
    pub fn from_vec(
        id_vec: Vec<String>,
        path_vec: Vec<String>,
        version_vec: Vec<String>,
    ) -> Vec<Self> {
        // First, make 3 to 1.
        let total: Vec<(String, String, String)> = id_vec
            .clone()
            .into_iter()
            .zip(path_vec.clone())
            .zip(version_vec.clone())
            .filter_map(|((a, b), c)| Some((a, b, c)))
            .collect();

        // Then iterate it
        let mut vector = Vec::new();
        for (id, path, version) in total {
            let package = Package::new(id, path, version);
            vector.push(package);
        }
        vector
    }
}

/// This function will read the configuration file and return a HashMap
///
/// The HashMap's key is the repository name, and the value is the repository URL
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
pub fn download(url: String, save: String, msg: Message) -> Result<(), Error> {
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

/// This function can help you to extract the package to the
/// temp dir.
///
/// It will return a String, and it is the output dir.
///
/// This will in use in some steps.
pub fn extract(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Extract first
    let output_dir = create_dir()?;
    let input_file = File::open(input_path)?;
    let decoder = XzDecoder::new(input_file);
    let mut archive = Archive::new(decoder);

    let output_path = Path::new(&output_dir);
    archive.unpack(output_path)?;

    // Convert to String
    let path_str = output_dir.to_string_lossy().into_owned();

    Ok(path_str)
}

// Create the temp dir to extract only
fn create_dir() -> Result<PathBuf, std::io::Error> {
    let mut rng = rand::rng();
    let mut random_suffix = String::new();
    let charset = "0123456789";
    for _ in 0..6 {
        let random_index = rng.random_range(0..charset.len());
        random_suffix.push(charset.chars().nth(random_index).unwrap());
    }

    let mut target_dir = PathBuf::from("/tmp");
    target_dir.push(format!("mcospkg{}", random_suffix));

    fs::create_dir(&target_dir)?;

    Ok(target_dir)
}
