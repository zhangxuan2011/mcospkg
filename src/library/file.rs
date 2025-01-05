use std::fs;
use std::fs::File;
use std::collections::HashMap;
use serde_json::{Value, Error};

// This function check is file exists. If yes, return true, else return false.
pub fn check_file_exist(filename: &str) -> bool {
    match File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}
