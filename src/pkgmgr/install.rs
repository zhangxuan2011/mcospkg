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
use crate::{
    Color, ErrorCode, Message, Package, PkgInfoToml, get_installed_package_info,
    set_executable_permission, set_installed_package_info,
};
use chrono::Local;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{File, copy, create_dir_all, remove_dir_all, remove_file, rename};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use walkdir::WalkDir;

/// This will help you to check the package type, and return
/// a string, which is the type of the package.
///
/// Of cource, you need to give this function the workdir, then
/// we can check it out.
///
/// This function will ONLY return 2 strings: "copy" and "build".
fn step1_check_pkg_instype(workdir: &str) -> String {
    // First, we needs to check is the package is valid.
    // Presets
    let build_script_path = format!("{}/BUILD-SCRIPT", workdir);
    let hooks_script_path = format!("{}/HOOKS", workdir);
    let unhooks_script_path = format!("{}/UNHOOKS", workdir);
    let build_path = Path::new(build_script_path.as_str());
    let hooks_path = Path::new(hooks_script_path.as_str());
    let unhooks_path = Path::new(unhooks_script_path.as_str());

    // Then check
    // First is the HOOKS and UNHOOKS
    if !hooks_path.exists() && !unhooks_path.exists() {
        return String::from("invalid");
    }

    // Then check the path
    if build_path.exists() {
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
        .template("{spinner:.green} {msg}")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(250));
    let package_msg = package_name.clone().cyan().bold();
    let total_msg: Message = format!("Building package \"{}\"...", package_msg).into();
    pb.set_message(total_msg.clone());

    // Then, preset some metadata
    let build_script_path = format!("{}/BUILD-SCRIPT", workdir);
    set_executable_permission(&build_script_path)?;

    // And set the log path
    let now = Local::now();
    let date_time_str = now.format("%Y-%m-%d-%H-%M-%S").to_string(); // YYYY-mm-dd-HH-MM-SS
    let log_name = format!("{}-{}.log", package_name, date_time_str);

    // Finally run the build script
    let command = format!(
        "{} > /dev/null 2> /var/log/mcospkg/{}",
        build_script_path, log_name
    );
    let status = Command::new("sh").arg("-c").arg(&command).status().unwrap();
    if !status.success() {
        return Err(ErrorCode::ExecuteError);
    }

    // Don't forget to copy the UNHOOKS file to the cureect place
    // The UNHOOKS's format should be like
    // /etc/mcospkg/database/remove_info/{PACKAGE_NAME}-UNHOOKS, which is a bash script.
    // So, move it
    let unhook_path = format!("{}/UNHOOKS", workdir);
    let unhook = Path::new(&unhook_path);
    if unhook.exists() {
        let place_to_unhook = format!(
            "/etc/mcospkg/database/remove_info/{}-UNHOOKS",
            package_name.clone()
        );
        let _ = rename(unhook_path, place_to_unhook);
    }

    register_package(package_name.clone(), version, dependencies)?;

    // Set the finish message
    let finish_msg: Message = format!("{} {}", total_msg, "Done".green().bold()).into();
    pb.finish_with_message(finish_msg);
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
    for (source, target) in paths_source.iter().zip(paths_target.iter()) {
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
    }

    // Remove something not good
    // The removing metadata
    let hooks = Path::new("/HOOKS");
    let unhooks = Path::new("/UNHOOKS");

    // First, run the hook
    if hooks.exists() {
        let build_script_path = "/HOOKS";
        set_executable_permission(build_script_path)?;

        let status = Command::new("sh")
            .arg("-c")
            .arg(build_script_path)
            .status()
            .unwrap();

        if !status.success() {
            return Err(ErrorCode::ExecuteError);
        }
    }

    // Second, copy the UNHOOKS to the currect dir
    // The UNHOOKS's format should be like
    // /etc/mcospkg/database/remove_info/{PACKAGE_NAME}-UNHOOKS, which is a bash script.
    // So, move it
    if unhooks.exists() {
        let place_to_unhook = format!("/etc/mcospkg/database/remove_info/{}-UNHOOKS", package_name);
        let _ = rename("/UNHOOKS", place_to_unhook);
    }

    // Main removing
    let _ = remove_file(hooks);
    let _ = remove_file(unhooks);

    // Store the file index
    path_index.retain(|s| !s.contains("/UNHOOKS") && !s.contains("/HOOKS")); // Delete something not good
    step3_copy_store_fileindex(package_name.clone(), path_index.clone());

    // Register the package information (use a function)
    register_package(package_name.clone(), version, dependencies)?;
    pb.finish();
    Ok(())
}

/// This function can register the package info, which is the
/// information of the installed package.
fn register_package(
    package: String,
    version: String,
    dependencies: Vec<String>,
) -> Result<(), ErrorCode> {
    // First, preset the data
    let mut pkginfo = get_installed_package_info();

    // Then, append the new package info to that install info
    let pkgtoml = PkgInfoToml {
        version,
        dependencies,
    };
    pkginfo.insert(package, pkgtoml);

    // Then write to that file
    set_installed_package_info(pkginfo);
    Ok(())
}

/// This function will store the package's file index, which
/// saved in a JSON format, in
/// /etc/mcospkg/database/remove_info/<PACKAGE_NAME>-index.json
///
/// NOTE: This function is for "copy" mode only
fn step3_copy_store_fileindex(package: String, file_index: Vec<String>) {
    // First, serialize the index
    let json_result = serde_json::to_string(&file_index).unwrap();

    // Then, we write that to the currect place
    let path = format!("/etc/mcospkg/database/remove_info/{}-index.json", package);

    let mut file_path = File::create(&path).unwrap();
    file_path.write_all(json_result.as_bytes()).unwrap();
}

/// The installing function
///
/// # Explain
/// This function will install package straightly, which provides
/// the most simple way.
///
/// Running without permissions won't successful, it will quit
/// because of PermissionDenied.
///
/// But be careful to install package with the unauthorized
/// package, it can probably break your system.
pub fn install_pkg(packages: Vec<Package>, workdirs: Vec<String>) -> Result<(), ErrorCode> {
    let color = Color::new();

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
        } else if pkg_instype == "copy" {
            step2_copy(
                package.id.clone(),
                package.version.clone(),
                package.dependencies.clone(),
                workdir.clone(),
            )?;
        } else {
            eprintln!(
                "{}: What the hell is that package called \"{}\"? It's invalid! So passed.",
                color.warning,
                package.id.clone()
            );
            continue;
        }

        // Clean up the directory and exit
        remove_dir_all(workdir).unwrap();
    }

    Ok(())
}
