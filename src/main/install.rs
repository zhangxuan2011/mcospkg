/// ## Information
/// Position: src/main/install.rs
/// Usage: The install library of src/main.rs
/// Date: 2025-02-10
/// Author: Xuan Zhang <zx20110412@outlook.com>
///
/// ## Description
/// This file is the library of src/main.rs.
/// It contains the struct `InstallData` and its methods.
///
///
/// ## Example
/// ```rust
/// mod main {
///     pub mod install;
/// }
/// use main::install;                              // Import the struct
/// // Main function
/// let mut install_data = install::InstallData::new();      // Create a new InstallData
/// install_data.step1_explain_pkg(pkglist);        // Explain the package (step 1)
/// install_data.step2_check_deps(pkglist);         // Check package dependencied (step 2)
/// install_data.step3_check_installed();           // Check if package has been installed (step 3)
/// install_data.step4_download();                  // Download package (step 4)
/// install_data.step5_install();                   // Install package (step 5)
/// ```
///
/// ## PS
/// The usage of this file is in src/main.rs.
/// Line 111-126 is the usage of this file.
/// (NOTE: The `src/main.rs` maybe update so that the lines may change.)
// Import some essential modules
use colored::Colorize;
use dialoguer::Input;
use mcospkg::{download, readcfg, Color, install_pkg};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::process::exit;

// =====json define area=====
// Define the pkg color.info
#[derive(Debug, Clone, Deserialize, Serialize)]
struct PkgInfo {
    filename: String,
    version: String,
    sha256sums: String,
}

// This implete the trait "Display", we'll use it later.
impl fmt::Display for PkgInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (version: {})", self.filename, self.version)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PkgIndex {
    // arch: String, // TODO: Support arch
    url: String,
    pkgindex: HashMap<String, PkgInfo>,
    baseon: HashMap<String, Vec<String>>,
    // group: HashMap<String, String>, // TODO: Support group
}

// =====Public information define area=====
pub struct InstallData {
    repoindex: Vec<(String, String)>, // The repository index
    url_total: Vec<String>,           // The repository url
    pkgindex_total: Vec<HashMap<String, PkgInfo>>, // The package index
    baseon_total: Vec<HashMap<String, Vec<String>>>, // The package baseon
    pkg_version_index: Vec<String>,   // The package version
    pkg_sha256sums_index: Vec<String>,  // The package sha256
    pkgindex: HashMap<String, PkgInfo>, // The package index
    fetch_index: Vec<String>,         // The package to fetch
    file_index: Vec<String>,          // The package to fetch
}

// Define Install Public Data
impl InstallData {
    pub fn new() -> Self {
        Self {
            // Initialize fields
            repoindex: vec![],
            url_total: vec![],
            pkgindex_total: vec![],
            baseon_total: vec![],
            pkg_version_index: vec![],
            pkg_sha256sums_index: vec![],
            pkgindex: HashMap::new(),
            fetch_index: vec![],
            file_index: vec![],
        }
    }

    pub fn step1_explain_pkg(&mut self, pkglist: Vec<String>) {
        let color = Color::new();
        print!("{}: Reading package index... ", color.info);

        // Stage 1: Explain the package
        // First, load configuration and get its HashMap
        match readcfg() {
            Err(e) => {
                println!("{}", color.error);
                println!("{}: {}", color.error, e);
                println!(
                    "{}: Consider using this format to write to that file:\n\t{}",
                    color.note,
                    "[reponame] = [repourl]".cyan()
                );
                exit(2)
            }
            Ok(repoconf) => {
                self.repoindex = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
            }
        }

        // Second, check if index is exist
        let _repopath: String = String::new(); //  We'll use it later
        let mut errtime = 0; // This will record the error times
        for (reponame, _) in &self.repoindex {
            let repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
            // If index not exist, just quit
            if !Path::new(&repopath).exists() {
                if errtime == 0 {
                    println!("{}", color.error);
                }
                println!(
                    "{}: Repository index \"{}\" not found",
                    color.error, reponame
                );
                errtime += 1;
            }
        }
        if errtime > 0 {
            println!(
                "{}: use \"{}\" to download it.",
                color.tip,
                "mcospkg-mirror update".cyan()
            );
            exit(1);
        }

        // And, read the PKGINDEX
        for (reponame, _) in &self.repoindex {
            // Read and parse the index
            let indexpath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
            let index_raw = std::fs::read_to_string(&indexpath).unwrap();
            let index: PkgIndex = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
                println!("{}", color.error);
                println!(
                    "{}: Invaild PKGINDEX format (In repository \"{}\")",
                    color.error, &reponame
                );
                println!(
                    "{}: Consider update the mirrorlist/mcospkg or contact the repository author.",
                    color.note
                );
                exit(1);
            });

            // Add them to the total
            self.url_total.push(index.url);
            self.pkgindex_total.push(index.pkgindex);
            self.baseon_total.push(index.baseon);
        }

        // Stage 1.5: Check if the package is exist
        self.pkgindex = HashMap::new(); // Initialize
        for (i, _) in self.pkgindex_total.iter().enumerate() {
            self.pkgindex.extend(self.pkgindex_total[i].clone());
        }

        // Main of this stage - Compare user input ("pkglist") and pkgindex
        for pkg in &pkglist {
            if !self.pkgindex.contains_key(pkg) {
                println!("{}", color.error);
                println!(
                    "{}: Package \"{}\" not found in any repositories.",
                    color.error, pkg
                );
                exit(2)
            }
        }
        println!("{}", color.done);
    }

    pub fn step2_check_deps(&mut self, pkglist: Vec<String>) {
        let color = Color::new();
        print!("{}: Checking package dependencies... ", color.info);

        // Stage 2: Check the packages' dependencies
        // To check it, we need to use the baseon_total
        // Cause the baseon_total is a vector, we need to use a loop to check it
        // First, we need to check if the package is exist in the baseon_total
        // If it is exist, we need to check if the package is exist in the baseon_total
        let mut baseon: HashMap<String, Vec<String>> = HashMap::new(); // The first string is the package name, and the second is the dependencies
        for (i, _) in self.baseon_total.iter().enumerate() {
            baseon.extend(self.baseon_total[i].clone());
        }
        // Generate a vector to record the packages that need dependencies
        let mut need_dependencies: Vec<String> = Vec::new(); // This will record them
        for pkg in &pkglist {
            if baseon.contains_key(pkg) {
                need_dependencies.push(pkg.clone());
            }
        }
        // Next, we need to check if the dependencies is exist in the pkgindex
        // If it is not exist, we need to quit
        for pkg in &need_dependencies {
            for dep in &baseon[pkg] {
                if !self.pkgindex.contains_key(dep) {
                    println!("{}", color.error);
                    println!(
                        "{}: Invaild package dependencies: \"{}\" (not found in package index)",
                        color.error, dep
                    );
                    exit(1)
                }
            }
        }

        // Finally, add them to the "fetch index"
        for pkg in &pkglist {
            self.fetch_index.push(pkg.clone());
        }

        for pkg in &need_dependencies {
            for dep in &baseon[pkg] {
                self.fetch_index.push(dep.clone());
            }
        }
        println!("{}", color.done);
    }

    pub fn step3_check_installed(&mut self, reinstall: bool, pkglist: Vec<String>) {
        let color = Color::new();
        print!("{}: Checking if the package is installed... ", color.info);

        // Stage 3: Check if the package is installed in the system
        // NOTE: If the "reinstall" = true, pass this stage
        // First, we need to check if the package is exist in the system
        // If it is exist, we need to ask user if they want to reinstall it
        // The method of checking is check if "/etc/mcospkg/database/remove_color.info/<package>-UNHOOKS" is exist
        // and check if the "/etc/mcospkg/database/packages.toml" has its information
        // If it is exist, we need to ask user if they want to reinstall it
        // If it is not exist, we need to ask user if they want to install it

        // First, read it
        let binding = fs::read_to_string("/etc/mcospkg/database/packages.toml")
            .unwrap_or_else(|err| {
                println!("{}", color.error);
                println!(
                    "{}: Cannot read \"/etc/mcospkg/database/packages.toml\": {}",
                    color.error, err
                );
                exit(1);
            })
            .to_string();
        let installed_packages = binding.split("\n").collect::<Vec<&str>>();

        // Then check
        let mut errtime = 0;
        for pkg in &pkglist {
            let check_pkg = format!("[{}]", &pkg);
            if !reinstall {
                for installed_pkg in installed_packages.clone() {
                    if installed_pkg == check_pkg {
                        if errtime == 0 {
                            println!("{}", color.error);
                        }
                        println!(
                            "{}: Package \"{}\" has installed, cannot reinstall it without \"reinstall\" mode",
                            color.error,
                            pkg,
                        );
                        errtime += 1;
                    } else {
                        continue;
                    }
                }
            }
        }

        if errtime > 0 {
            println!(
                "{}: To reinstall it, please append an argument \"{}\" after the command.",
                color.note,
                "-r".cyan()
            );
            exit(1);
        }
        println!("{}", color.done);
    }

    pub fn step4_download(&mut self, bypass_ask: bool) {
        let color = Color::new();

        // Stage 4: Download the package
        // First, get package's version
        // Then, we need to ask user that if they want to install it
        println!(
            "{}: The following packages is being installing:",
            color.info
        );
        let len = self.fetch_index.len();
        for (i, pkg) in self.fetch_index.clone().into_iter().enumerate() {
            // Get each package's version
            let pkg_version = self.pkgindex.get(&pkg).unwrap().version.clone();
            self.pkg_version_index.push(pkg_version.clone());

            // Get each package's sha256
            let pkg_sha256sums = self.pkgindex.get(&pkg).unwrap().sha256sums.clone();
            self.pkg_sha256sums_index.push(pkg_sha256sums.clone());
            // Print the package list
            if i < len - 1 {
                print!("{} ({}), ", pkg, pkg_version);
            } else {
                print!("{} ({})", pkg, pkg_version);
            }
        }
        println!("");

        if !bypass_ask {
            let input: String = Input::new()
                .with_prompt("\nProceed to install these packages? (y/n)")
                .interact_text()
                .unwrap();
            if input != "y" && input != "Y" {
                println!("{}: User rejected the installation request", color.error);
                exit(1);
            }
        } else {
            println!("\nProceed to install these packages? (y/n): y");
        }

        // Now user allowed to install packages by using mcospkg. Congratulations :) !
        // Well, do you still remember the repo url we got? Let's use it!
        // Then, we need to create a directory to store the packages
        // So we need to check if the directory is exist
        // If it is not exist, we need to create it
        println!("{}: Downloading packages... ", color.info);
        let cache_path = "/var/cache/mcospkg";
        if !Path::new(cache_path).exists() {
            std::fs::create_dir(cache_path).unwrap();
        } else if !Path::new(cache_path).is_dir() {
            println!(
                "{}: The cache path is not a directory. Please make it to a dir",
                color.error
            );
            exit(1);
        }

        // Now, we need to download the packages
        // The package store rules:
        // <repository url>/<package name>/<package file>
        // For example:
        //  https://zhangxuan2011.github.io/mcospkg/repo/main/mcospkg/mcospkg-0.1.1-x86_64.tar.xz
        //  ------------------------------------------------- ------- ---------------------------
        //                   repository url                   pkgname      package file
        //
        // Note: in different screens, this example shows different effects.
        // So, we need to download the package file and store it in the cache path
        // How to download? use the library we've imported - download.
        // Define something
        let mut pkg_msgs: Vec<&'static str> = Vec::new(); // This will record the message of downloading

        for pkgname in &self.fetch_index {
            let pkg_msg = format!("{}", pkgname);
            let pkg_msg = Box::leak(pkg_msg.into_boxed_str());
            pkg_msgs.push(pkg_msg);
        }

        for (pkg, msg) in self
            .fetch_index
            .clone()
            .into_iter()
            .zip(pkg_msgs.into_iter())
            .clone()
        {
            // Get the repo url
            let repo_url = self.url_total.iter().next().unwrap().clone();

            // And, get the pkg name
            let pkg_name = pkg.clone();

            // And, get the pkg file
            let pkg_file = self.pkgindex.get(&pkg).unwrap().filename.clone();

            // Now, we need to generate its path and url
            let pkg_url = format!("{}/{}/{}", repo_url, pkg_name, pkg_file);
            let pkg_path = format!("{}/{}", cache_path, pkg_file);

            // Download the package
            let mut errtime: u32 = 0;
            if let Err(e) = download(pkg_url, pkg_path.clone(), &msg) {
                println!("{}: {}", color.error, e);
                errtime += 1;
            }

            if errtime > 0 {
                println!(
                    "{}: Cannot download some packages, installation abort.",
                    color.error
                );
                println!(
                    "{}: Please check your network connection or contact the author",
                    color.note
                );
                exit(1);
            }
            // And, add it to the file index - use it later
            self.file_index.push(pkg_path.clone());
        }
    }
    
    pub fn step5_check_sums(&mut self) {
        let color = Color::new();

        // Stage 5: Check sha256sums
        // This stage check sha256sums of the package.
        // All sums are stored in "self.pkg_sha256sums_total",
        // so we'll get it first.
        println!("{}: Checking SHA256 sums...", color.info);

        // Get each sha256sums
        let mut errtime: u32 = 0;
        for (sha256, pkg) in self
            .pkg_sha256sums_index
            .clone()
            .into_iter()
            .zip(self.fetch_index.clone().into_iter())
            .clone()
        {
            print!("{} \"{}\": ", "Vaildating package".cyan().bold(), pkg.clone());
            // First, get the file's sha256 intergrity.
            // Get the file name
            let file = self.pkgindex.get(&pkg).unwrap().filename.clone();
            // Get the full path
            let full_path = format!("/var/cache/mcospkg/{}", file);
            // Then calculate its sums
            let file_sums = Self::vaildate_sums(&full_path).unwrap();
            // Check
            if file_sums != sha256 {
                println!("{}", color.no);
                errtime += 1;
            } else {
                println!("{}", color.ok);
            }
        }

        if errtime > 0 {
            println!("{}: {} packages does not pass the vaildating.", color.error, errtime);
            exit(1)
        }
    }

    pub fn step6_install(&mut self) {
        let color = Color::new();
        println!("{}: Installing packages... ", color.info);

        // Stage 6: Install the package
        // My friend, Xiaokuai, uses C to write the install library.
        // I'll thank him at here :)
        // So, we need to use the C library to install the package
        // First, we need to convert the string to CString
        let mut c_file_index: Vec<CString> = Vec::new(); // Record the index, we'll use it
    
        // Convert the string to CString
        for filepath in &self.file_index {
            let c_pkg = CString::new(filepath.clone()).unwrap();
            c_file_index.push(c_pkg);
        }
        // Convert version_total to CString
        let mut c_version_total: Vec<CString> = Vec::new();
        for version in &self.pkg_version_index {
            let c_version = CString::new(version.clone()).unwrap();
            c_version_total.push(c_version);
        }
        // Then, we need to use the C library to install the package
        let version_and_file = c_file_index.iter().zip(c_version_total.iter());
        for (pkg, c_version) in version_and_file {
            let c_pkg_name = CString::new(
                pkg.to_str()
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap()
                    .split("-")
                    .next()
                    .unwrap(),
            )
            .unwrap();
            let c_pkg_path = CString::new(pkg.to_str().unwrap()).unwrap();
            let status = unsafe {
                install_pkg(
                    c_pkg_path.as_ptr(), 
                    c_pkg_name.as_ptr(), 
                    c_version.as_ptr()
                )
            };
            if status != 0 {
                println!("{}: The installation didn't exit normally.", color.error);
            }
        }
    }

    fn vaildate_sums(file_path: &str) -> io::Result<String> {
        // This uses in vaildating sha256sums of the things
        // we downloaded.
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}
