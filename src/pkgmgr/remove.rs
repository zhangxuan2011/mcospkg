pub fn remove_pkg(packages: Vec<String>) -> i32 {
    for i in packages {
        println!("Package {} is being removed", i);
    }
    0
}
