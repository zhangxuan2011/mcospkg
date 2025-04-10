use crate::{ErrorCode, Package};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{Permissions, remove_dir_all, set_permissions};
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
fn step2_build(package_name: &str, workdir: &str) -> Result<(), ErrorCode> {
    // First, preset some metadata
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

    Ok(())
}

/// The step 2, but the mode is "copy".
/// This will copy the file to the root directory "/" to continue the installation.
fn step2_copy(package_name: String, workdir: String) {
    // Set up the progress bar
    let mut pb = ProgressBar::new(100);
    let style = ProgressStyle::default_bar()
        .template("{msg} {eta_precise} [{bar:40.green/blue}] {percent}%")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    let package_msg: std::borrow::Cow<'static, str> = package_name.clone().into();
    pb.set_message(package_msg);

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
    for path in path_index_raw {
        // TODO: Rewrite the logic later.
        if let Some(index) = workdir.find(pattern) {
            // 如果找到了模式，截取模式之后的内容
            let result = path[index + 13..].to_string();
            path_index.push(result);
        }
    }

    println!("All files: {:?}", path_index);
    pb.finish();
}

pub fn install_pkg(packages: Vec<Package>, workdirs: Vec<String>) -> Result<(), ErrorCode> {
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
            step2_build(&package.id, &workdir)?;
        } else {
            step2_copy(package.id.clone(), workdir.clone());
        }

        // Clean up the directory and exit
        remove_dir_all(workdir).unwrap();
    }

    Ok(())
}
