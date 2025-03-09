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
use mcospkg::{get_installed_package_info, remove_pkg, Color, PkgInfoToml};
use std::collections::HashMap;
use std::ffi::CString;
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

    pub fn step2_check_deps(&mut self, pkglist: Vec<String>) {
        let color = Color::new();

        // Stage 2: Check the dependencies
        // Get its keys
        let mut package_keys: Vec<String> = Vec::new();
        for (key, _) in self.package.iter() {
            package_keys.push(key.clone());
        }

        // Make sure the specified the package is exist in that file
        // Check the HashMap's key is ok.
        let mut errtime = 0;
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
        for pkg in &pkglist {
            for dep in &self.package[pkg].dependencies {
                dependencies.push(dep.clone());
            }
        }
        println!("{}", color.done);

        // Merge them
        self.delete_pkgs.append(&mut pkglist.clone());
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
                .with_prompt("\nDo you want to continue? (y/n)")
                .interact_text()
                .unwrap();
            if input != "y" && input != "Y" {
                println!("{}: User rejected the uninstallation request", color.error);
                exit(1);
            }
        } else {
            println!("\nADo you proceed to remove these packages? (y/n): y");
        }
    }

    pub fn step4_remove(&self) {
        // Stage 4: Remove the package
        for delete_pkg in &self.delete_pkgs {
            let package_name = CString::new(delete_pkg.as_str()).unwrap();
            unsafe {
                remove_pkg(package_name.as_ptr());
            };
        }
    }
}
