/// ## Information
/// Position: src/main/remove.rs
/// Usage: The remove library of src/main.rs
/// Date: 2025-02-10
/// Author: Xuan Zhang <zx20110412@outlook.com>
///
/// ## Description
/// This file is the library of src/main.rs.
/// It contains the struct `RemoveData` and its methods.
///
///
/// ## Example
/// ```rust
/// mod main {
///     pub mod remove;
/// }
/// use main::remove;                              // Import the struct
/// // Main function
/// let mut remove_data = remove::RemoveData::new();      // Create a new InstallData
/// remove_data.step1_explain_pkg(pkglist);        // Explain the package (step 1)
/// remove_data.step2_check_deps(pkglist);         // Check package dependencied (step 2)
/// remove_data.step3_check_installed();           // Check if package has been installed (step 3)
/// remove_data.step4_download();                  // Download package (step 4)
/// remove_data.step5_install();                   // Install package (step 5)
/// ```
///
/// ## PS
/// The usage of this file is in src/main.rs.
/// Line 137-150 is the usage of this file.
/// (NOTE: The `src/main.rs` maybe update so that the lines may change.)
///
// Import some modules
use dialoguer::Input;
use mcospkg::{Color, PkgInfoToml, get_installed_package_info, rust_remove_pkg};
use std::collections::{HashMap, HashSet};
use std::process::exit;

// ========structs define area=========
pub struct RemoveData {
    delete_pkgs: Vec<String>,
    package: HashMap<String, PkgInfoToml>,
}

impl RemoveData {
    pub fn new() -> Self {
        Self {
            delete_pkgs: vec![],
            package: HashMap::new(),
        }
    }

    pub fn step1_explain_pkg(&mut self) {
        // Stage 1: Explain the package
        // In "Remove" function, the most important is the dependencies.
        // In "/etc/mcospkg/packages.toml", in each file's dependencies, defined it.
        // For example:
        /* [package_name]
           version = "0.1.1"
           dependencies = [
               "dep1",
               "dep2",
               "dep3",
               ...,
               "depn"
           ]
        */
        // Parse it
        self.package = get_installed_package_info();
    }

    pub fn step2_check_deps(&mut self, mut pkglist: Vec<String>) {
        let color = Color::new();

        // Stage 2: Check the dependencies
        // Get its keys
        let mut package_keys: Vec<String> = Vec::new();
        for (key, _) in self.package.iter() {
            package_keys.push(key.clone());
        }
        let mut visited = HashSet::new();

        // Make sure the specified the package is exist in that file
        // Check the HashMap's key is ok.
        let mut errtime: u32 = 0;
        for package in &pkglist {
            if !package_keys.contains(package) {
                println!(
                    "{}: Package \"{}\" is not installed, so we have no idea (T_T)",
                    color.error, package
                );
                errtime += 1;
            }
        }

        if errtime > 0 {
            println!("{}: {} errors occurred, terminated.", color.error, errtime);
            exit(1)
        }

        // Then let's see see...
        print!("{}: Resolving dependencies... ", color.info);

        // Read the vector "dependencies"
        let mut dependencies: Vec<String> = Vec::new();
        errtime = 0; // Reset error times
        for pkg in &pkglist {
            self.check_all_dependencies(
                pkg,
                &package_keys,
                &mut dependencies,
                &mut errtime,
                &mut visited,
                &color,
            );
        }
        if errtime > 0 {
            println!(
                "{}: Perhaps you modified the package information?",
                color.tip
            );
            exit(1)
        }
        println!("{}", color.done);

        // Merge them
        self.delete_pkgs.append(&mut pkglist);
        self.delete_pkgs.append(&mut dependencies);
    }

    pub fn step3_ask_user(&self, bypass_ask: bool) {
        let color = Color::new();

        // Stage 3: Ask user
        println!("{}: The following packages will be removed:", color.info);
        for pkg in &self.delete_pkgs {
            print!("{} ", pkg);
        }
        println!(); // Make sure it can show normally

        if !bypass_ask {
            let input: String = Input::new()
                .with_prompt("\nProceed to remove these packages? (y/n)")
                .interact_text()
                .unwrap();
            if input != "y" && input != "Y" {
                println!("{}: User rejected the uninstallation request.", color.error);
                exit(1);
            }
        } else {
            println!("\nProceed to remove these packages? (y/n): y");
        }
    }

    pub fn step4_remove(&self) {
        let color = Color::new();

        // Stage 4: Remove the package
        let mut packages: Vec<String> = Vec::new();
        for delete_pkg in self.delete_pkgs.clone() {
            packages.push(delete_pkg);
        }
        let status = rust_remove_pkg(packages);
        if status != 0 {
            println!("{}: The uninstallation didn't exit normally.", color.error);
        }
    }

    // Check the dependencies completely
    fn check_all_dependencies(
        &self,
        pkg: &str,
        package_keys: &Vec<String>,
        dependencies: &mut Vec<String>,
        errtime: &mut u32,
        visited: &mut HashSet<String>,
        color: &Color,
    ) {
        if visited.contains(pkg) {
            return;
        }
        visited.insert(pkg.to_string());

        for dep in &self.package[pkg].dependencies {
            if !package_keys.contains(dep) {
                if *errtime == 0 {
                    println!("{}", color.failed);
                }
                println!(
                    "{}: Invalid dependencies \"{}\" in package \"{}\".",
                    color.error, dep, pkg
                );
                *errtime += 1;
            } else {
                dependencies.push(dep.clone());
                self.check_all_dependencies(
                    dep,
                    package_keys,
                    dependencies,
                    errtime,
                    visited,
                    color,
                );
            }
        }
    }
}
