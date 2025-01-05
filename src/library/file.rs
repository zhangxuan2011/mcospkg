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

pub fn decode_json(file_path: &str) -> Result<HashMap<&str, &str>, Box<dyn std::error::Error>> {
    // 读取文件内容，如果读取失败直接返回错误
    let content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };

    // 尝试将字符串解析为JSON值，如果解析失败返回错误
    let json_value: Result<Value, Error> = serde_json::from_str(&content);
    let json_value = match json_value {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    // 判断JSON值是否是对象类型（对应HashMap的结构）
    if let Value::Object(map) = json_value {
        // 使用filter_map来处理键值对，确保值是字符串类型的情况才构建到新的HashMap中
        let hash_map: HashMap<&str, &str> = map
            .iter()
            .filter_map(|(k, v)| match v {
                Value::String(s) => Some((k.as_str(), s)),
                _ => None,
            })
            .collect();
        Ok(hash_map)
    } else {
        // 如果不是对象类型，返回相应错误提示
        Err("Not a json file format".into())
    }
}
