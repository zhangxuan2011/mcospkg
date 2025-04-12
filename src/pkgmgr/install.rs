/// This file will install the packages and register the
/// package info.
///
/// Actually, the main install function was re-exported to
/// the "install_pkg" in this crate, renamed to
/// "rust_install_pkg".
///
/// And the others? they are the steps of installing package,
/// which is very important.
///
/// For more usages, see the doc in "src/lib.rs"
// Import some modules
use crate::{ErrorCode, Message, Package, PkgInfoToml, get_installed_package_info};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs::{Permissions, copy, create_dir_all, remove_dir_all, remove_file, set_permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

/// This will help you to check the package type, and return
/// a string, which is the type of the package.
///
/// Of cource, you need to give this function the workdir, then
/// we can check it out.
///
/// This function will ONLY return 2 strings: "copy" and "build".
fn step1_check_pkg_instype(workdir: &str) -> String {
    // First, we needs to check the file "BUILD-SCRIPT" exists
    // If yes, return "src", else return "build".

    // Open the file
    let build_script_path = format!("{}/BUILD-SCRIPT", workdir);
    let path = Path::new(build_script_path.as_str());

    // Then check
    if path.exists() {
        String::from("build")
    } else {
        String::from("copy")
    }
}

/// This is the step 2, but for build only.
///
/// It will return a "Result" type, which is the enum of
/// "ErrorCode" declared in crate "mcospkg".
fn step2_build(
    package_name: String,
    version: String,
    dependencies: Vec<String>,
    workdir: String,
) -> Result<(), ErrorCode> {
    // First, Create a progress bar.
    let pb = ProgressBar::new_spinner();
    let style = ProgressStyle::default_spinner()
        .template("{spinner} {msg} {percent}%")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    let package_msg: Message = package_name.clone().into();
    pb.set_message(package_msg);

    // Then, preset some metadata
    let permission = Permissions::from_mode(0o755);
    let build_script_path = format!("{}/BUILD-SCRIPT", workdir);
    let binding = build_script_path.clone();
    let path = Path::new(binding.as_str());

    // Then set the file permission
    if let Err(_) = set_permissions(path, permission) {
        return Err(ErrorCode::PermissionDenied);
    }

    // And set the log path
    let now = Local::now();
    let date_time_str = now.format("%Y-%m-%d-%H-%M-%S").to_string(); // YYYY-mm-dd-HH-MM-SS
    let log_name = format!("{}-{}.log", package_name, date_time_str);

    // Finally run the build script
    let command = format!(
        "{} > /dev/null 2> /var/log/mcospkg/{}",
        build_script_path, log_name
    );
    let status = Command::new("sh").arg("-c").arg(command).status().unwrap();
    if !status.success() {
        return Err(ErrorCode::ExecuteError);
    }

    register_package(version, dependencies)?;
    Ok(())
}

/// The step 2, but the mode is "copy".
/// This will copy the file to the root directory "/" to continue the installation.
fn step2_copy(
    package_name: String,
    version: String,
    dependencies: Vec<String>,
    workdir: String,
) -> Result<(), ErrorCode> {
    // Get all file and append it to a vector
    let mut path_index_raw: Vec<String> = Vec::new(); // Store it
    for entry in WalkDir::new(&workdir) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    path_index_raw.push(path_str.to_string());
                }
            }
        }
    }

    // Delete something unusual
    // First, delete the string before "workdir"
    let mut path_index: Vec<String> = Vec::new();
    let pattern = "mcospkg";
    for path in &path_index_raw {
        if let Some(index) = workdir.find(pattern) {
            // Strip the unused path
            let result = path[index + 13..].to_string();
            path_index.push(result);
        }
    }

    // Next, copy the file to the currect dirs.
    //
    // Parse it to the Path format
    let paths_source: Vec<&Path> = path_index_raw.iter().map(|s| Path::new(s)).collect();
    let paths_target: Vec<&Path> = path_index.iter().map(|s| Path::new(s)).collect();

    // Set up the progress bar
    let total_files = paths_source.clone().len();
    let pb = ProgressBar::new(total_files as u64);
    let style = ProgressStyle::default_bar()
        .template("{msg} {eta_precise} [{bar:40.green/blue}] {percent}%")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    let package_msg: Message = package_name.clone().into();
    pb.set_message(package_msg);

    // Start to copy
    for (source, target) in paths_source.into_iter().zip(paths_target.into_iter()) {
        // Create the parent directory
        if let Some(parent) = target.parent() {
            if let Err(_) = create_dir_all(parent) {
                return Err(ErrorCode::CreateDirError);
            }
        }

        // Then copy them
        match copy(source, target) {
            Ok(_) => {
                pb.inc(1);
            }
            Err(_) => {
                return Err(ErrorCode::CopyFilesError);
            }
        }

        // Remove something not good
        // The removing metadata
        let hooks = Path::new("/HOOKS");
        let unhooks = Path::new("/UNHOOKS");

        // Main removing
        let _ = remove_file(hooks);
        let _ = remove_file(unhooks);

        // Set up the new length
        let new_total = pb.length().unwrap() + 10;
        pb.set_length(new_total);
    }

    // Register the package information (use a function)
    register_package(version, dependencies)?;
    pb.finish();
    Ok(())
}

/// This function can register the package info, which is the
/// information of the installed package.
fn register_package(version: String, dependencies: Vec<String>) -> Result<(), ErrorCode> {
    // First, preset the data
    let pkginfo = get_installed_package_info();

    // Then read the info file
    Ok(())
}

pub fn install_pkg(packages: Vec<Package>, workdirs: Vec<String>) -> Result<(), ErrorCode> {
    println!("{:#?}", packages);
    // Iterate the index and set the ProgressBar
    for (package, workdir) in packages.into_iter().zip(workdirs) {
        // Then call the installing steps
        let pkg_instype = step1_check_pkg_instype(&workdir); // Get the install type

        // Next we needs to chdir
        if let Err(_) = std::env::set_current_dir(&workdir) {
            return Err(ErrorCode::ChangeDirError);
        }

        // Do the next step
        if pkg_instype == "build" {
            step2_build(
                package.id.clone(),
                package.version.clone(),
                package.dependencies.clone(),
                workdir.clone(),
            )?;
        } else {
            step2_copy(
                package.id.clone(),
                package.version.clone(),
                package.dependencies.clone(),
                workdir.clone(),
            )?;
        }

        // Clean up the directory and exit
        remove_dir_all(workdir).unwrap();
    }

    Ok(())
}
