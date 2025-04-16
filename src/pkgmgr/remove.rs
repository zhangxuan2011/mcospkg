use crate::{Color, ErrorCode, Message, set_executable_permission, get_installed_package_info, set_installed_package_info};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

/* =====JSON format area===== */
// This only used for parsing the remove files into a vector.
#[derive(Debug, Deserialize)]
struct RemoveFiles(Vec<String>);

fn step1_check_instype(package: String) -> String {
    // Actually, we only need to check is 1 file exists:
    // /etc/mcospkg/database/remove_info/{PACKAGE_NAME}-index.json
    //
    // If exist, it's easy.
    let path_raw = format!("/etc/mcospkg/database/remove_info/{}-index.json", package);
    let path = Path::new(&path_raw);
    if path.exists() {
        String::from("copy")
    } else {
        String::from("build")
    }
}

fn step2_copy_rmfile(package: String) -> Result<(), ErrorCode> {
    let color = Color::new();

    // First, parse the json file as a vector.
    let path = format!("/etc/mcospkg/database/remove_info/{}-index.json", package);
    let rminfo_raw = std::fs::read_to_string(&path).unwrap();
    let remove_info: RemoveFiles = serde_json::from_str(&rminfo_raw).unwrap(); // Main parsing code

    // Then create a Progress bar
    let pb = ProgressBar::new(package.len() as u64);
    let style = ProgressStyle::default_bar()
        .template("{msg} {eta_precise} [{bar:40.green/blue}] {percent}%")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    let package_msg: Message = package.clone().into();
    pb.set_message(package_msg);

    // Finally iterate it to delete
    for file in remove_info.0 {
        if let Err(why) = remove_file(&file) {
            eprintln!(
                "{}: Cannot remove \"{}\": {}, passed.",
                color.warning, file, why
            );
            continue;
        };
        pb.inc(1);
    }

    // Finished the progress bar
    pb.finish();

    // Finally, delete the index file
    if let Err(why) = remove_file(&path) {
        eprintln!(
            "{}: Cannot remove the index file: {}",
            color.error, why
        );
    }

    Ok(())
}

fn step3_unregister_package(package: String) -> Result<(), ErrorCode> {
    let color = Color::new();

    // Get the package info first
    let mut map = get_installed_package_info();

    // Delete the package in dependencies
    // Get the version and dependencies
    for pkginfo in map.values_mut() {
        let new_deps = pkginfo.dependencies.clone().into_iter()
           .filter(|pkg| *pkg != package)
           .collect();
        pkginfo.dependencies = new_deps;
    }

    // Then delete the package
    if let None = map.remove(&package) {
        eprintln!("{}: Cannot unregister the package because it even doesn't exist!", color.error);
        return Err(ErrorCode::UnregisterError);
    }
    
    // Finally, set it up
    set_installed_package_info(map);

    Ok(())
}

/// The remove function for the package.
///
/// # Explain
/// This function can remove a package straightly, which needs
/// the sudo privilege.
///
/// Also, it's a dangerous thing if you're removing something
pub fn remove_pkg(packages: Vec<String>) -> Result<(), ErrorCode> {
    let color = Color::new();

    // Iterate it to see
    for package in packages {
        // First, we needs to check the package install type
        let instype = step1_check_instype(package.clone());

        // If the install type is "copy", remove with file list
        // defined at "/etc/mcospkg/database/remove_info/{PACKAGE_NAME}-index.json"
        if instype == "copy" {
            step2_copy_rmfile(package.clone())?;
        }

        // Then, run the package's unhooks.
        let place_to_unhook = format!(
            "/etc/mcospkg/database/remove_info/{}-UNHOOKS",
            package
        );

        // Set up permission
        set_executable_permission(&place_to_unhook)?;

        // Then run
        let status = Command::new("sh").arg("-c").arg(&place_to_unhook).status().unwrap();
        if !status.success() {
            return Err(ErrorCode::ExecuteError);
        }

        // Clean up itself
        if let Err(why) = remove_file(place_to_unhook) {
            eprintln!(
                "{}: Cannot remove the unhook file: {}",
                color.error, why
            );
        }

        // Finally, unregister the package info
        step3_unregister_package(package)?;
    }
    Ok(())
}
