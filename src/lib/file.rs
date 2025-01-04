use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;

// This function check is file exists. If yes, return true, else return false.
pub fn check_file_exist(filename: &str) -> bool {
    match File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn decode_json(
    file_path: &str,
) -> Result<HashMap<&str, &str>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let json_value: Value = serde_json::from_str(&content)?;
    if let Some(map) = json_value.as_object() {
        let hash_map: HashMap<&str, &str> =
            map.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        Ok(hash_map)
    } else {
        Err("Not a json file format".into())
    }
}
