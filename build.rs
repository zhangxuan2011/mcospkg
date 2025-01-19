use std::path::Path;
use cc;

fn main() {
    let src_dir = Path::new("src");
    let include_dir = Path::new("include");
    let out_dir = Path::new("target/lib");

    // Check if the include directory exists
    if !include_dir.exists() {
        panic!("Include directory does not exist");
    }

    let mut build = cc::Build::new();
    build.flag("--std=c99");
    build.flag("-D_GNU_SOURCE");
    for entry in src_dir.read_dir().expect("Failed to read src directory") {
        let entry = entry.expect("Failed to read entry in src directory");
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "c" {
            build.file(path);
        }
    }

    // Configure and build the pkgmgr library
    build.include(include_dir);
    build.out_dir(&out_dir);
    build.compile("pkgmgr");

    // Tell Rust to link against the pkgmgr library
    println!("cargo:rustc-link-lib=static=pkgmgr");
    println!("cargo:rustc-link-search=native=lib");
}
