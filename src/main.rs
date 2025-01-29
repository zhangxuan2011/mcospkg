// Now, we need to import some modules:
use clap::Parser;
use colored::Colorize;
use dialoguer::Input;
use libc::{c_char, c_int};
use mcospkg::download;
use mcospkg::readcfg;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::path::Path;
use std::process::exit;

// Import C Libs(libpkgmgr.a)
extern "C" {
    fn installPackage(package_path: *const c_char, package_name: *const c_char) -> c_int;
    fn removePackage(package_name: *const c_char); // TODO: use it in Remove function
}

// ========structs define area=========

// ====Arguments define area====
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.1-debug")]

// Define argument lists
struct Args {
    #[arg(required = true, help = "Supports: install/remove/update/reinstall")]
    option: String,

    #[arg(required = false)]
    packages: Vec<String>,

    #[arg(
        long = "bypass",
        short = 'y',
        default_value_t = false,
        help = "Specify it will not ask ANY questions"
    )]
    bypass_ask: bool,
}

// =====json define area=====
// Define the pkg info
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

// ========functions define area==========
fn main() {
    let error = "error".red().bold();
    let args = Args::parse();
    match args.option.as_str() {
        "install" => install(args.packages, args.bypass_ask, false),
        "remove" => remove(args.packages, args.bypass_ask),
        "reinstall" => install(args.packages, args.bypass_ask, true),
        _ => println!("{}: unknown option: {}", error, args.option),
    };
}

fn install(pkglist: Vec<String>, bypass_ask: bool, reinstall: bool) {
    // Presets
    let error = "error".red().bold();
    let tip = "tip".green().bold();
    let info = "info".blue().bold();
    let done = "Done".green().bold();
    let note = "note".yellow().bold();

    // Tell user is this mode is "reinstall"
    if reinstall {
        println!("{}: Reinstall mode enabled.", note);
    }

    // Stage 1: Explain the package
    // First, load configuration and get its HashMap
    let repoindex: Vec<(String, String)>; // This will record the repository index. First String is its name, and the second is its url.
    match readcfg() {
        Err(e) => {
            println!("{}: {}", error, e);
            println!(
                "{}: Consider using this format to write to that file:\n\t{}",
                note,
                "[reponame] = [repourl]".cyan()
            );
            exit(2)
        }
        Ok(repoconf) => {
            repoindex = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
        }
    }

    // Second, check if index is exist
    let _repopath: String = String::new(); //  We'll use it later
    let mut errtime = 0; // This will record the error times
    for (reponame, _) in &repoindex {
        let repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        // If index not exist, just quit
        if !Path::new(&repopath).exists() {
            println!("{}: Repository index \"{}\" not found", error, reponame);
            errtime += 1;
        }
    }
    if errtime > 0 {
        println!(
            "{}: use \"{}\" to download it.",
            tip,
            "mcospkg-mirror update".cyan()
        );
        exit(1);
    }

    // Next, check if pkgindex is empty
    if pkglist.len() <= 0 {
        println!("{}: No package(s) specified.", error);
        exit(2);
    }

    // And, read the PKGINDEX
    print!("{}: Reading package index... ", info);
    let mut url_total: Vec<String> = Vec::new(); // This will record the "url"
    let mut pkgindex_total = Vec::new(); // This will record the "pkgindex"
    let mut baseon_total = Vec::new(); // This will record the "baseon"
    for (reponame, _) in &repoindex {
        // Read and parse the index
        let indexpath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        let index_raw = std::fs::read_to_string(&indexpath).unwrap();
        let index: PkgIndex = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
            println!("{}", error);
            eprintln!("{}: Invaild PKGINDEX format (In repository {})", error, &reponame);
            eprintln!("{}: Consider update the mirrorlist or contact the repository author.", note);
            exit(1);
        });

        // Add them to the total
        url_total.push(index.url);
        pkgindex_total.push(index.pkgindex);
        baseon_total.push(index.baseon);
    }
    println!("{}", done);

    // Stage 2: Check if package is exist
    let mut pkgindex: HashMap<String, PkgInfo> = HashMap::new();
    for (i, _) in pkgindex_total.iter().enumerate() {
        pkgindex.extend(pkgindex_total[i].clone());
    }

    // Main of this stage - Compare user input ("pkglist") and pkgindex
    for pkg in &pkglist {
        if !pkgindex.contains_key(pkg) {
            println!("{}", error);
            eprintln!(
                "{}: Package \"{}\" not found in any repositories.",
                error, pkg
            );
            exit(2)
        }
    }

    // Stage 2: Check the packages' dependencies
    // To check it, we need to use the baseon_total
    // Cause the baseon_total is a vector, we need to use a loop to check it
    // First, we need to check if the package is exist in the baseon_total
    // If it is exist, we need to check if the package is exist in the baseon_total
    print!("{}: Checking package dependencies... ", info);
    let mut baseon: HashMap<String, Vec<String>> = HashMap::new(); // The first string is the package name, and the second is the dependencies
    for (i, _) in baseon_total.iter().enumerate() {
        baseon.extend(baseon_total[i].clone());
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
            if !pkgindex.contains_key(dep) {
                println!("{}", error);
                println!(
                    "{}: Invaild package dependencies: \"{}\" (not found in package index)",
                    error, dep
                );
                exit(1)
            }
        }
    }

    // Finally, add them to the "fetch index"
    let mut fetch_index: Vec<String> = Vec::new(); // This is all the package we needs to fetch in the next stage
    for pkg in &pkglist {
        fetch_index.push(pkg.clone());
    }

    for pkg in &need_dependencies {
        for dep in &baseon[pkg] {
            fetch_index.push(dep.clone());
        }
    }

    println!("{}", done);

    // Stage 3: Check if the package is installed in the system
    // NOTE: If the "reinstall" = true, pass this stage
    // First, we need to check if the package is exist in the system
    // If it is exist, we need to ask user if they want to reinstall it
    // The method of checking is check if "/etc/mcospkg/database/remove_info/<package>-UNHOOKS" is exist
    // If it is exist, we need to ask user if they want to reinstall it
    // If it is not exist, we need to ask user if they want to install it
    if !reinstall {
        print!("{}: Checking if the package is installed... ", info);
        for pkg in &fetch_index {
            if Path::new(&format!(
                "/etc/mcospkg/database/remove_info/{}-UNHOOKS",
                pkg
            ))
            .exists()
            {
                println!("{}", error);
                println!(
                    "{}: Package \"{}\" is installed, cannot reinstall it\nTo reinstall it, use \"{}\"",
                    error,
                    pkg,
                    "mcospkg reinstall <package>".cyan()
                );
                exit(1)
            }
        }
    }

    // If the "reinstall" = true, check is it NOT installed
    if reinstall {
        print!("{}: Checking if the package is installed... ", info);
        for pkg in &fetch_index {
            if !Path::new(&format!(
                "/etc/mcospkg/database/remove_info/{}-UNHOOKS",
                pkg
            ))
            .exists()
            {
                println!("{}", error);
                println!(
                    "{}: Package \"{}\" is not installed, cannot reinstall it without \"reinstall\" mode.\nTo install it, use \"{}\"",
                    error,
                    pkg,
                    "mcospkg install <package>".cyan()
                );
                exit(1)
            }
        }
    }
    println!("{}", done);

    // Stage 4: Download the package
    // First, we need to ask user that if they want to install it
    println!("{}: The following packages is being installed:", info);
    for pkg in &fetch_index {
        print!("{} ", pkg);
    }
    println!("");

    if !bypass_ask {
        let input: String = Input::new()
            .with_prompt("\nDo you want to continue? (y/n)")
            .interact_text()
            .unwrap();
        if input != "y" && input != "Y" {
            println!("{}: User rejected the installation request", error);
            exit(1);
        }
    } else {
        println!("\nDo you want to continue? (y/n): y");
    }

    // Now user allowed to install packages by using mcospkg. Congratulations :) !
    // Well, do you still remember the repo url we got? Let's use it!
    // Then, we need to create a directory to store the packages
    // So we need to check if the directory is exist
    // If it is not exist, we need to create it
    println!("{}: Downloading packages... ", info);
    let cache_path = "/var/cache/mcospkg";
    if !Path::new(cache_path).exists() {
        std::fs::create_dir(cache_path).unwrap();
    } else if !Path::new(cache_path).is_dir() {
        println!(
            "{}: The cache path is not a directory. Please make it to a dir",
            error
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

    for pkgname in &fetch_index {
        let pkg_msg = format!("{}", pkgname);
        let pkg_msg = Box::leak(pkg_msg.into_boxed_str());
        pkg_msgs.push(pkg_msg);
    }

    for (pkg, msg) in fetch_index.into_iter().zip(pkg_msgs.into_iter()).clone() {
        // Get the repo url
        let repo_url = url_total.iter().next().unwrap().clone();

        // And, get the pkg name
        let pkg_name = pkg.clone();

        // And, get the pkg file
        let pkg_file = pkgindex.get(&pkg).unwrap().filename.clone();

        // Then, get package's version
        let pkg_version = pkgindex.get(&pkg).unwrap().version.clone();

        // Now, we need to generate its path and url
        let pkg_url = format!("{}/{}/{}", repo_url, pkg_name, pkg_file);
        let pkg_path = format!("{}/{}", cache_path, pkg_file);

        // Download the package
        let mut errtime: u32 = 0;
        if let Err(e) = download(pkg_url, pkg_path.clone(), &msg) {
            println!("{}: {}", error, e);
            errtime += 1;
        }

        if errtime > 0 {
            println!(
                "{}: Cannot download some packages, installation abort.",
                error
            );
            println!(
                "{}: Please check your network connection or contact the author",
                note
            );
            exit(1);
        }
        // And, add it to the file index - use it later
        file_index.push(pkg_path.clone());
        pkg_version_index.push(pkg_version.clone());
    }

    // Stage 5: Install the package
    // My friend, Xiaokuai, uses C to write the install library.
    // I'll thank him at here :)
    // So, we need to use the C library to install the package
    // First, we need to convert the string to CString
    println!("{}: Installing packages... ", info);
    let mut c_file_index: Vec<CString> = Vec::new(); // Record the index, we'll use it
    for filepath in &file_index {
        let c_pkg = CString::new(filepath.clone()).unwrap();
        c_file_index.push(c_pkg);
    }
    // Then, we need to use the C library to install the package
    for pkg in &c_file_index {
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
        let res = unsafe { installPackage(c_pkg_path.as_ptr(), c_pkg_name.as_ptr()) };
        if res != 0 {
            println!("{}: {}", error, res);
        }
    }

    // And, that's complete!
}

// This is the install function
// =====S====P====L====I====T=====
// This is the remove function

fn remove(pkglist: Vec<String>, _bypass_ask: bool) {
    for pkg in pkglist {
        let package_name = CString::new(pkg.as_str()).unwrap();
        unsafe { removePackage(package_name.as_ptr()) }
    }
}
