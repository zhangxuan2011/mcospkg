pub fn remove_pkg(package: Vec<String>) -> i32 {
    for i in package {
        println!("Package {} is being removed", i);
    }
    0
}
