use crate::Package;

pub fn install_pkg(package: Vec<Package>) -> i32 {
    println!("This is the package info:");
    println!("{:#?}", package);
    0
}
