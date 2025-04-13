use crate::ErrorCode;

pub fn remove_pkg(packages: Vec<String>) -> Result<(), ErrorCode> {
    for i in packages {
        println!("Package {} is being removed", i);
    }
    Ok(())
}
