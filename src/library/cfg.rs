use std::collections::HashMap;
use std::fs;

pub fn readcfg() -> HashMap<String, String> {
    // First, read the configuration
    let mut repoconf_raw = fs::read_to_string("/etc/mcospkg/repo.conf").expect("Failed to open \"/etc/mcospkg/repo.conf\".");

    // Second, make it cleaner
    repoconf_raw = repoconf_raw.replace(" ", "").replace("\t", "");

    // Third, we convert it to the HashMap
    let mut repoconf: HashMap<String, String> = HashMap::new();
    for line in repoconf_raw.lines() {
        if let Some((key, value)) = line.split_once('=') {
            repoconf.insert(key.to_string(), value.to_string());
        }
    }

    // Finally, return it
    repoconf
}
