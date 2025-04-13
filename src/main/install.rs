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
use mcospkg::{Color, Message, Package, download, extract, readcfg, rust_install_pkg};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Read, Write};
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
    pkg_sha256sums_index: Vec<String>, // The package sha256
    pkgindex: HashMap<String, PkgInfo>, // The package index
    fetch_index: Vec<String>,         // The package to fetch
    file_index: Vec<String>,          // The package to fetch
    workdir_index: Vec<String>,       // The workdir index
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
            workdir_index: vec![],
        }
    }

    pub fn step1_explain_pkg(&mut self, pkglist: Vec<String>) {
        let color = Color::new();
        print!("{}: Reading package index... ", color.info);
        io::stdout().flush().unwrap();

        // Stage 1: Explain the package
        // First, load configuration and get its HashMap
        match readcfg() {
            Err(e) => {
                println!("{}", color.failed);
                eprintln!("{}: {}", color.error, e);
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
                if errtime == 0 {
                    println!("{}", color.failed);
                }
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
                println!("{}", color.failed);
                eprintln!(
                    "{}: Invaild PKGINDEX format (In repository \"{}\")",
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
                println!("{}", color.failed);
                eprintln!(
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
        io::stdout().flush().unwrap();

        // Stage 2: Check the packages' dependencies
        // To check it, we need to use the baseon_total
        // Cause the baseon_total is a vector, we need to use a loop to check it
        // First, we need to check if the package is exist in the baseon_total
        // If it is exist, we need to check if the package is exist in the baseon_total
        let mut baseon: HashMap<String, Vec<String>> = HashMap::new(); // The first string is the package name, and the second is the dependencies
        for (i, _) in self.baseon_total.iter().enumerate() {
            baseon.extend(self.baseon_total[i].clone());
        }
        let mut visited = HashSet::new(); // Will record the deps of checked.
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
                    println!("{}", color.failed);
                    eprintln!(
                        "{}: Invaild package dependencies: \"{}\" (not found in package index)",
                        color.error, dep
                    );
                    exit(1)
                } else {
                    self.check_all_dependencies(&dep.clone(), &mut visited);
                }
            }
        }

        // Finally, add them to the "fetch index"
        let mut added_pkgs = HashSet::new(); //  Record the package append to fetch_index
        for pkg in &pkglist {
            if added_pkgs.insert(pkg.clone()) {
                self.fetch_index.push(pkg.clone());
            }
        }

        for pkg in &need_dependencies {
            for dep in &baseon[pkg] {
                if added_pkgs.insert(dep.clone()) {
                    self.fetch_index.push(dep.clone());
                    self.check_all_dependencies(dep, &mut added_pkgs);
                }
            }
        }
        println!("{}", color.done);
    }

    pub fn step3_check_installed(&mut self, reinstall: bool) {
        let color = Color::new();
        let mut processed_pkgs = HashSet::new();

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
                println!("{}", color.failed);
                eprintln!(
                    "{}: Cannot read \"/etc/mcospkg/database/packages.toml\": {}",
                    color.error, err
                );
                exit(1);
            })
            .to_string();
        let installed_packages = binding.split("\n").collect::<Vec<&str>>();

        // Then check
        for pkg in self.fetch_index.clone() {
            let check_pkg = format!("[{}]", &pkg); // Convert with the TOML format
            if !reinstall {
                for installed_pkg in installed_packages.clone() {
                    if installed_pkg == check_pkg {
                        if processed_pkgs.insert(pkg.clone()) {
                            println!(
                                "{}: Package \"{}\" has installed, but it's not reinstall mode now, ignored.",
                                color.warning, pkg,
                            );
                        }
                        if let Some(index) = self.fetch_index.iter().position(|x| *x == *pkg) {
                            self.fetch_index.remove(index);
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    pub fn step4_download(&mut self, bypass_ask: bool) {
        let color = Color::new();
        let len = self.fetch_index.len();

        // Stage 4: Download the package
        // If len == 0, it means that no package will be installed.
        // Have a check :0
        if len == 0 {
            eprintln!("{}: No any package will be installed.", color.error);
            eprintln!(
                "{}: Maybe some packages has been ignored? If yes, add the argument \"{}\".",
                color.tip,
                "-r".cyan()
            );
            exit(1)
        }

        // Then, we need to ask user that if they want to install it
        println!("{}: The following packages is being installed:", color.info);

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
                eprintln!("{}: User rejected the installation request", color.error);
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
            eprintln!(
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
        let mut pkg_msgs: Vec<Message> = Vec::new(); // This will record the message of downloading

        for pkgname in &self.fetch_index {
            let pkg_msg: Message = pkgname.clone().into();
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
            if let Err(e) = download(pkg_url, pkg_path.clone(), msg) {
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
            print!("{} \"{}\": ", "Package".bold(), pkg.cyan().bold().clone());
            io::stdout().flush().unwrap();
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
            println!(
                "{}: {} packages does not pass the vaildating.",
                color.error, errtime
            );
            exit(1)
        }
    }

    pub fn step6_extract(&mut self) {
        let color = Color::new();
        println!("{}: Extracting packages... ", color.info);

        // Stage 6: Extract the package
        // In the installation, we needs to extract the packages, is it [doge]
        // and we needs to extract it to a defined place,
        // Such as /var/cache/mcospkg (dafault)
        // In it, we only needs to use 1 function, use it later.

        // Iterate the id and path
        for (id, path) in self
            .fetch_index
            .clone()
            .into_iter()
            .zip(self.file_index.clone())
        {
            print!("{} \"{}\"... ", "Extracting".bold(), id.cyan().bold());
            io::stdout().flush().unwrap();
            let workdir = extract(&path).unwrap_or_else(|err| {
                println!("{}", color.failed);
                eprintln!("{}: Cannot extract packages: {}", color.error, err);
                exit(1)
            });
            println!("{}", color.done);
            self.workdir_index.push(workdir);
        }
    }

    pub fn step7_install(&mut self) {
        let color = Color::new();
        println!("{}: Installing packages... ", color.info);

        // Stage 7: Install the package
        // My friend, Xiaokuai, Helps me to write the install library.
        // I'll thank him at here :)
        // So, we need to use hsi library to install the package
        // First, get the dependencies
        let mut dependencies: Vec<Vec<String>> = Vec::new();
        let length_baseon = self.baseon_total.len();
        for i in 0..length_baseon {
            let map = &self.baseon_total[i];
            for j in 0..map.keys().len() {
                let pkg = &self.fetch_index[j];
                if let Some(deps) = map.get(pkg) {
                    dependencies.push(deps.clone());
                } else {
                    dependencies.push(Vec::new());
                }
            }
        }

        // Make sure the length is the same or larger than others
        for _ in length_baseon..self.baseon_total.len() {
            dependencies.push(Vec::new());
        }

        for _ in length_baseon..self.fetch_index.len() {
            dependencies.push(Vec::new());
        }

        // Then, convert them to struct "Package".
        // Make 3 vectors as 1 vector
        let packages = Package::from_vec(
            self.fetch_index.clone(),
            self.file_index.clone(),
            dependencies.clone(),
            self.pkg_version_index.clone(),
        );

        let status = rust_install_pkg(packages, self.workdir_index.clone());
        if let Err(error) = status {
            eprintln!(
                "{}: The installation has received an error, \"{:?}\".",
                color.error, error
            );
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

    fn check_all_dependencies(&mut self, dep: &str, added_pkgs: &mut HashSet<String>) {
        // Check is the deps has processed
        if added_pkgs.contains(dep) {
            return;
        }
        added_pkgs.insert(dep.to_string());

        // Get the next deps
        let sub_deps: Vec<String> = self
            .baseon_total
            .iter()
            .flat_map(|m| m.get(dep).map(|v| v.clone()).unwrap_or_default())
            .collect();
        for sub_dep in sub_deps {
            if !self.pkgindex.contains_key(&sub_dep) {
                let color = Color::new();
                println!("{}", color.failed);
                eprintln!(
                    "{}: Dependency \"{}\" of \"{}\" has an invalid sub - dependency \"{}\".",
                    color.error,
                    dep,
                    self.fetch_index.last().unwrap(),
                    sub_dep
                );
                exit(1);
            } else {
                if added_pkgs.insert(sub_dep.clone()) {
                    self.fetch_index.push(sub_dep.clone());
                }
                self.check_all_dependencies(&sub_dep, added_pkgs);
            }
        }
    }
}
