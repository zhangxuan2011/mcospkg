use mcospkg::{download, readcfg, Color};
use colored::Colorize;
use dialoguer::Input;
use libc::{c_char, c_int};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::path::Path;
use std::process::exit;

// =====json define area=====
// Define the pkg color.info
#[derive(Debug, Clone, Deserialize, Serialize)]
struct PkgInfo {
    filename: String,
    version: String,
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
    version_total: Vec<String>,       // The package version
    pkgindex: HashMap<String, PkgInfo>, // The package index
    fetch_index: Vec<String>,         // The package to fetch
    file_index: Vec<String>,          // The package to fetch
}

// ==========Extern area==========
extern "C" {
    fn installPackage(
        package_path: *const c_char,
        package_name: *const c_char,
        version: *const c_char,
    ) -> c_int;
}

impl InstallData {
    pub fn new() -> Self {
        Self {
            // Initialize fields
            repoindex: vec![],
            url_total: vec![],
            pkgindex_total: vec![],
            baseon_total: vec![],
            version_total: vec![],
            pkgindex: std::collections::HashMap::new(),
            fetch_index: vec![],
            file_index: vec![],
        }
    }

    pub fn step1(&mut self) {
        let color = Color::new();
        // Stage 1: Explain the package
        // First, load configuration and get its HashMap
        match readcfg() {
            Err(e) => {
                println!("{}: {}", color.error, e);
                eprintln!(
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
                eprintln!(
                    "{}: Repository index \"{}\" not found",
                    color.error, reponame
                );
                errtime += 1;
            }
        }
        if errtime > 0 {
            eprintln!(
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
                eprintln!(
                    "{}: Invaild PKGINDEX format (In repository {})",
                    color.error, &reponame
                );
                eprintln!(
                    "{}: Consider update the mirrorlist/mcospkg or contact the repository author.",
                    color.note
                );
                exit(1);
            });

            // Add them to the total
            self.url_total.push(index.url);
            self.version_total.push(
                index
                    .pkgindex
                    .get("version")
                    .unwrap_or_else(|| {
                        eprintln!(
                            "{}: Invaild PKGINDEX format (In repository {})",
                            color.error, &reponame
                        );
                        eprintln!(
                    "{}: Consider update the mirrorlist/mcospkg or contact the repository author.",
                    color.note
                );
                        exit(1);
                    })
                    .to_string(),
            );
            self.pkgindex_total.push(index.pkgindex);
            self.baseon_total.push(index.baseon);
        }
    }

    pub fn step2(&mut self, pkglist: Vec<String>) {
        let color = Color::new();
        // Stage 2: Check if the package is exist
        self.pkgindex = HashMap::new(); // Initialize
        for (i, _) in self.pkgindex_total.iter().enumerate() {
            self.pkgindex.extend(self.pkgindex_total[i].clone());
        }

        // Main of this stage - Compare user input ("pkglist") and pkgindex
        for pkg in &pkglist {
            if !self.pkgindex.contains_key(pkg) {
                println!("{}", color.error);
                eprintln!(
                    "{}: Package \"{}\" not found in any repositories.",
                    color.error, pkg
                );
                exit(2)
            }
        }
    }

    pub fn step3(&mut self, pkglist: Vec<String>) {
        let color = Color::new();

        // Stage 3: Check the packages' dependencies
        // To check it, we need to use the baseon_total
        // Cause the baseon_total is a vector, we need to use a loop to check it
        // First, we need to check if the package is exist in the baseon_total
        // If it is exist, we need to check if the package is exist in the baseon_total
        print!("{}: Checking package dependencies... ", color.info);
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
    }

    pub fn step4(&mut self, reinstall: bool) {
        let color = Color::new();

        // Stage 4: Check if the package is installed in the system
        // NOTE: If the "reinstall" = true, pass this stage
        // First, we need to check if the package is exist in the system
        // If it is exist, we need to ask user if they want to reinstall it
        // The method of checking is check if "/etc/mcospkg/database/remove_color.info/<package>-UNHOOKS" is exist
        // If it is exist, we need to ask user if they want to reinstall it
        // If it is not exist, we need to ask user if they want to install it
        if !reinstall {
            print!("{}: Checking if the package is installed... ", color.info);
            for pkg in &self.fetch_index {
                if Path::new(&format!(
                    "/etc/mcospkg/database/remove_color.info/{}-UNHOOKS",
                    pkg
                ))
                .exists()
                {
                    println!("{}", color.error);
                    eprintln!(
                        "{}: Package \"{}\" is installed, cannot reinstall it\nTo reinstall it, use \"{}\"",
                        color.error,
                        pkg,
                        "mcospkg reinstall <package>".cyan()
                    );
                    exit(1)
                }
            }
        }

        // If the "reinstall" = true, check is it NOT installed
        if reinstall {
            print!("{}: Checking if the package is installed... ", color.info);
            for pkg in &self.fetch_index {
                if !Path::new(&format!(
                    "/etc/mcospkg/database/remove_color.info/{}-UNHOOKS",
                    pkg
                ))
                .exists()
                {
                    println!("{}", color.error);
                    eprintln!(
                        "{}: Package \"{}\" is not installed, cannot reinstall it without \"reinstall\" mode.\nTo install it, use \"{}\"",
                        color.error,
                        pkg,
                        "mcospkg install <package>".cyan()
                    );
                    exit(1)
                }
            }
        }
    }

    pub fn step5(&mut self, bypass_ask: bool) {
        let color = Color::new();

        // Stage 4: Download the package
        // First, we need to ask user that if they want to install it
        println!("{}: The following packages is being installed:", color.info);
        for pkg in &self.fetch_index {
            print!("{} ", pkg);
        }
        println!("");

        if !bypass_ask {
            let input: String = Input::new()
                .with_prompt("\nDo you want to continue? (y/n)")
                .interact_text()
                .unwrap();
            if input != "y" && input != "Y" {
                println!("{}: User rejected the installation request", color.error);
                exit(1);
            }
        } else {
            println!("\nADo you proceed to install these packages? (y/n): y");
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
        let mut file_index: Vec<String> = Vec::new(); // Record the index, we'll use it in the next stage
        let mut pkg_msgs: Vec<&'static str> = Vec::new(); // This will record the message of downloading
        let mut pkg_version_index = Vec::new(); // This will record the package's version

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

            // Then, get package's version
            let pkg_version = self.pkgindex.get(&pkg).unwrap().version.clone();

            // Now, we need to generate its path and url
            let pkg_url = format!("{}/{}/{}", repo_url, pkg_name, pkg_file);
            let pkg_path = format!("{}/{}", cache_path, pkg_file);

            // Download the package
            let mut errtime: u32 = 0;
            if let Err(e) = download(pkg_url, pkg_path.clone(), &msg) {
                eprintln!("{}: {}", color.error, e);
                errtime += 1;
            }

            if errtime > 0 {
                eprintln!(
                    "{}: Cannot download some packages, installation abort.",
                    color.error
                );
                eprintln!(
                    "{}: Please check your network connection or contact the author",
                    color.note
                );
                exit(1);
            }
            // And, add it to the file index - use it later
            file_index.push(pkg_path.clone());
            pkg_version_index.push(pkg_version.clone());
        }
    }

    pub fn step6(&mut self) {
        let color = Color::new();

        // Stage 6: Install the package
        // My friend, Xiaokuai, uses C to write the install library.
        // I'll thank him at here :)
        // So, we need to use the C library to install the package
        // First, we need to convert the string to CString
        println!("{}: Installing packages... ", color.info);
        let mut c_file_index: Vec<CString> = Vec::new(); // Record the index, we'll use it

        // Convert the string to CString
        for filepath in &self.file_index {
            let c_pkg = CString::new(filepath.clone()).unwrap();
            c_file_index.push(c_pkg);
        }

        // Convert version_total to CString
        let mut c_version_total: Vec<CString> = Vec::new();
        for version in &self.version_total {
            let c_version = CString::new(version.clone()).unwrap();
            c_version_total.push(c_version);
        }

        // Then, we need to use the C library to install the package
        for (pkg, c_version) in c_file_index.iter().zip(c_version_total.iter()) {
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
            let res = unsafe {
                installPackage(c_pkg_path.as_ptr(), c_pkg_name.as_ptr(), c_version.as_ptr())
            };
            if res != 0 {
                println!("{}: {}", color.error, res);
            }
        }
    }
}
