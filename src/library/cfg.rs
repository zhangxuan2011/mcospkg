use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn readcfg() -> Result<HashMap<String, String>, Error> {
    // First, read the configuration
    let mut repoconf_raw = fs::read_to_string("/etc/mcospkg/repo.conf").map_err(|_| {
        Error::new(
            ErrorKind::Other,
            "Repository config file \"/etc/mcospkg/repo.conf\" not found",
        )
    })?;
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
    Ok(repoconf)
}
