use std::path::{Path, PathBuf};
use cc;

fn main() {
    let src_dir = Path::new("src");
    let include_dir = Path::new("include");
    let out_dir = PathBuf::from("target/release");

    let mut build = cc::Build::new();
    for entry in src_dir.read_dir().expect("Failed to read src directory") {
        let entry = entry.expect("Failed to read entry in src directory");
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "c" {
            build.file(path);
        }
    }
    build.include(include_dir);
    build.static_flag(true);
    build.out_dir(&out_dir);
    build.compile("pkgmgr");
}

