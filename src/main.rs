// First, we should declare the lib
mod library {
    pub mod cfg;
    pub mod download;
}

// Now, we need to import some modules:
use crate::library::cfg::readcfg;
use crate::library::download::download;
use clap::Parser;
use colored::Colorize;
use dialoguer::Input;
use serde_json;
use serde::{Serialize, Deserialize};
use std::process::exit;
use std::collections::HashMap;
use std::io::Write;
use std::ffi::CString;
use std::path::Path;
use libc::{c_char, c_int};

// Import C Libs(libpkgmgr.a)
extern "C" {
    fn installPackage(package_path: *const c_char, package_name: *const c_char) -> c_int;
    // fn removePackage(package_name: *const c_char) -> c_int;  // TODO: use it in Remove function
}

// Configure parser
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.1-debug")]

// Define argument lists
struct Args {
    #[arg(required = true, help = "Supports: install/remove/update")]
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

// Define the json format
#[derive(Debug, Serialize, Deserialize)]
struct PkgIndex {
    // arch: Option<String>, // TODO: Support arch
    url: Option<String>,
    pkgindex: Option<HashMap<String, String>>,
    baseon: Option<HashMap<String, Vec<String>>>,
    // group: Option<HashMap<String, String>>, // TODO: Support group
}

fn main() {
    let error = "error".red().bold();
    let args = Args::parse();
    match args.option.as_str() {
        "install" => install(args.packages),
        "remove" => remove(args.packages),
        _ => println!("{}: unknown option: {}", error, args.option),
    };
}

fn install(pkglist: Vec<String>) {
    let error = "error".red().bold();
    let tip = "tip".green().bold();
    let info = "info".blue().bold();
    let done = "Done".green().bold();
    let args = Args::parse();

    // Stage 1: Explain the package
    // First, load configuration and get its HashMap
    let repoindex: Vec<(String, String)>;   // This will record the repository index. First String is its name, and the second is its url.
    match readcfg() {
        Err(e) => {
            println!("{}: {}", error, e);
            println!(
                "{}: Consider using this format to write to that file:\n\t{}",
                "note".bold().green(),
                "[reponame] = [repourl]".cyan()
            );
            exit(2)
        },
        Ok(repoconf) => {
            repoindex = repoconf.into_iter().map(|(k, v)| (k, v)).collect();
        }
    }

    // Second, check if index is exist
    let _repopath: String = String::new();  //  We'll use it later
    let mut errtime = 0;    // This will record the error times
    for (reponame, _) in &repoindex {
        let repopath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        // If index not exist, just quit
        if! Path::new(&repopath).exists() {
            println!(
                "{}: Repository index \"{}\" not found",
                error, reponame
            );
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
    let mut pkgindex_total= Vec::new(); // This will record the "pkgindex"
    let mut baseon_total = Vec::new(); // This will record the "baseon"
    for (reponame, _) in &repoindex {
        // Read and parse the index
        let indexpath = format!("/etc/mcospkg/database/remote/{}.json", reponame);
        let index_raw = std::fs::read_to_string(&indexpath).unwrap();
        let index: PkgIndex = serde_json::from_str(&index_raw).unwrap();

        // Push it to the vector
        url_total.push(index.url.unwrap());
        pkgindex_total.push(index.pkgindex.unwrap());
        baseon_total.push(index.baseon.unwrap());
    }
    // Stage 2: Check if package is exist
    let mut pkgindex: HashMap<String, String> = HashMap::new();     // The first string is the package name, and the second is the file name
    for (i, _) in pkgindex_total.iter().enumerate() {
        pkgindex.extend(pkgindex_total[i].clone());
    }

    // Main of this stage - Compare user input ("pkglist") and pkgindex
    for pkg in &pkglist {
        if! pkgindex.contains_key(pkg) {
            println!("{}", error);
            println!("{}: Package \"{}\" not found in any repositories.", error, pkg);
            exit(2)
        }
    }
    println!("{}", done);

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
            if! pkgindex.contains_key(dep) {
                println!("{}", error);
                println!("{}: Invaild package dependencies: \"{}\" (not found in package index)", error, dep);
                exit(1)
            }
        }
    }

    // Finally, add them to the "fetch index"
    let mut fetch_index: Vec<String> = Vec::new();      // This is all the package we needs to fetch in the next stage
    for pkg in &pkglist {
        fetch_index.push(pkg.clone());
    }

    for pkg in &need_dependencies {
        for dep in &baseon[pkg] {
            fetch_index.push(dep.clone());
        }
    }
    
    println!("{}", done);
    // Stage 3: Download the package
    // First, we need to ask user that if they want to install it
    println!("{}: The following packages is being installed:", info);
    for pkg in &fetch_index {
        print!("{} ", pkg);
    }
    std::io::stdout().flush().unwrap();

    if! args.bypass_ask {
        let input: String = Input::new()
            .with_prompt("\n\nDo you want to continue? (y/n)")
            .interact_text()
            .unwrap();
        if input != "y" && input!= "Y" {
            println!("{}: User rejected the installation request", error);
            exit(1);
        }
    } else {
        println!("\n\nDo you want to continue? (y/n): y");
    }

    // Now user allowed to install packages by using mcospkg. Congratulations :) !
    // Well, do you still remember the repo url we got? Let's use it!
    // Then, we need to create a directory to store the packages
    // So we need to check if the directory is exist
    // If it is not exist, we need to create it
    println!("{}: Downloading packages... ", info);
    let cache_path = "/var/cache/mcospkg/";
    if !Path::new(cache_path).exists() {
        std::fs::create_dir(cache_path).unwrap();
    } else if !Path::new(cache_path).is_dir() {
        println!("{}: The cache path is not a directory. Please make it to a dir", error);
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
    // Now we needs to get the repo url
    let repo_url: String = String::new();   // Declare it
    let pkg_name: String = String::new();   // Declare it
    let pkg_file: String = String::new();   // Declare it

    let mut file_index: Vec<String> = Vec::new();   // Record the index, we'll use it in the next stage
    for _ in &fetch_index {
        for url in &url_total {
            // Get the repo url
            #[allow(unused_variables)]
            let repo_url = url;
        }

        // And, get the pkg name
        for pkg in &fetch_index {
            // Get the pkg name
            #[allow(unused_variables)]
            let pkg_name = pkg;
        }

        // And, get the pkg file
        for pkg in &fetch_index {
            // Get the pkg file
            #[allow(unused_variables)]
            let pkg_file = pkgindex.get(pkg).unwrap();
        }

        // Now, we need to download the package
        
        let pkg_url = format!("{}/{}/{}", repo_url, pkg_name, pkg_file);
        let pkg_path = format!("{}/{}/{}", cache_path, pkg_name, pkg_file);
        // Download the package
        if let Err(e) = download(pkg_url, pkg_path.clone(), "Downloading package...") {
            println!("{}: {}", error, e);
        }
        // And, add it to the file index
        file_index.push(pkg_path.clone());
    }
    // Stage 4: Install the package
    // My friend, Xiaokuai, uses C to write the install library.
    // I'll thank him at here :)
    // So, we need to use the C library to install the package
    // First, we need to convert the string to CString
    let mut c_file_index: Vec<CString> = Vec::new();   // Record the index, we'll use it
    for filepath in &file_index {
        let c_pkg = CString::new(filepath.clone()).unwrap();
        c_file_index.push(c_pkg);
    }
    // Then, we need to use the C library to install the package
    for pkg in &c_file_index {
        let c_pkg_name = CString::new(pkg.to_str().unwrap().split("/").last().unwrap().split("-").next().unwrap()).unwrap();
        let c_pkg_path = CString::new(pkg.to_str().unwrap()).unwrap();
        let res = unsafe {
            installPackage(
                c_pkg_path.as_ptr(),
                c_pkg_name.as_ptr()
            ) 
        };
        if res != 0 {
            println!("{}: {}", error, res);
        }
    }
}

// This is the install function
// =====S====P====L====I====T=====
// This is the remove function

fn remove(_pkglist: Vec<String>) {}
