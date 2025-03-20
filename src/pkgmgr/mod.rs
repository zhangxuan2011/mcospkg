/// # Description
///
/// This crate is the install and remove function, rewrite to
/// Rust from C.
///
/// # Example
/// This is the example of install and remove:
///
/// ```rust
/// use mcospkg::{
///     rust_install_pkg, 
///     rust_remove_pkg, 
///     Package
/// };  // Use this function
/// 
/// // Define some basic information
/// let name = String::from("example");  // Package ID
/// let path = String::from("/path/to/pkg"); // Package path
/// let version = String::from("0.1.0");    // Package version
///
/// // Install package
/// let package = Package {
///     id: name,
///     path,
///     version,
/// };  // Convert to the struct "Package"
///
/// let packages: Vec<Package> = vec![package]; // Append it to Vector
///
/// let _ = rust_install_pkg(packages); // Use it, will return i32
/// 
/// // Next, remove it.
/// let packages: Vec<String> = vec![name];
///
/// // Use it
/// let _ = rust_remove_pkg(packages);
///
/// ```
///
// The Re-export area

mod install;
mod remove;

pub use install::install_pkg;
pub use remove::remove_pkg;
