//! This file is used in testing the method of the struct
//! `Package`'s methods.
//!
//! To use that, please follow the instruction of the struct 
//! `Package` with `cargo doc --no-deps --open`.
// Import the library
use mcospkg::Package;

// Define the public value
fn public() -> Package {
    let id = "mcospkg".to_string();
    let path = "/path/to/mcospkg.tar.xz".to_string();
    let version = "0.9.1".to_string();

    // Main test, use that func
    let package = Package::new(id, path, version);
    package
}

// Test 1. Check is the function "new" returns the currect 
// value.
#[test]
fn is_new_function_returns_correct_value() {
    let package = public();

    // Assert it
    assert_eq!(package.id, "mcospkg");
    assert_eq!(package.path, "/path/to/mcospkg.tar.xz");
    assert_eq!(package.version, "0.9.1");
}

// Test 2. Check is it ok to return a vector
#[test]
fn is_to_vec_result_returns_currect_vector() {
    let package = public();

    // Then try to use that method
    let packages = package.clone().to_vec();
    let packages_compare = vec![package];
    assert_eq!(packages, packages_compare);
}

// Test 3. Check can we use 3 vecs to return the Vec<Package>
#[test]
fn can_three_vecs_returns_one() {
    // Prepare some vecs
    let ids = vec!["mcospkg".to_string()];
    let paths = vec!["/path/to/mcospkg.tar.xz".to_string()];
    let versions = vec!["0.9.1".to_string()];

    // Use that func
    let packages = Package::from_vec(ids, paths, versions);
    let package = public(); // Get package metadata
    let package_compare = vec![package];    // Make it to vec
    assert_eq!(packages, package_compare);
}
