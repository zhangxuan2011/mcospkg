use std::fs::File;

// This function check is file exists. If yes, return true, else return false.
pub fn check_file_exist(filename: &str) -> bool {
    match File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}
