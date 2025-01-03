use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Preset结构体用于表示预设配置信息，类似于Python中的Preset类。
// 它包含配置目录、使用方标识以及用于存储仓库相关信息的向量等字段。
struct Preset {
    config_dir: String,
    used_by: String,
    repos: Vec<String>,
    reponame: Vec<String>,
    repourl: Vec<String>,
}

impl Preset {
    // Preset结构体的构造函数。
    // 使用给定的配置目录和使用方标识来初始化一个新的Preset实例，类似于Python类中的__init__方法。
    fn new(config_dir: &str, used_by: &str) -> Preset {
        Preset {
            config_dir: config_dir.to_string(),
            used_by: used_by.to_string(),
            repos: Vec::new(),
            reponame: Vec::new(),
            repourl: Vec::new(),
        }
    }

    // 此方法用于检查仓库配置文件是否存在。
    // 如果文件在指定的配置目录中存在，则返回true，否则返回false。
    fn check_is_repocfg_exist(&self) -> bool {
        Path::new(&format!("{}/repo.conf", self.config_dir)).exists()
    }

    // 此方法从`repos`向量中拆分出仓库名称和仓库URL。
    // 它利用Rust中的迭代器按特定间隔提取元素，并将它们收集到分别用于存储仓库名称和URL的独立向量中。
    fn split_repo_name_url(&mut self) {
        self.reponame = self.repos.iter().step_by(2).cloned().collect();
        self.repourl = self.repos.iter().skip(1).step_by(2).cloned().collect();
    }

    // 此方法检查仓库配置文件是否存在并执行相关处理。
    // 如果文件存在，它会逐行读取文件，将每行拆分成多个部分，并将这些部分存储到`repos`向量中。
    // 同时会处理向量末尾可能出现空字符串的情况。
    // 如果一切正常则返回0，若出现诸如文件未找到或处理过程中其他错误等问题则返回-1。
    fn check_repo_conf_exist(&mut self) -> i32 {
        if self.check_is_repocfg_exist() {
            if let Ok(file) = File::open(format!("{}/repo.conf", self.config_dir)) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line_str) = line {
                        let parts = line_str.trim().split('=').collect::<Vec<&str>>();
                        self.repos.extend(parts.iter().map(|s| s.to_string()));
                    }
                }
                if self.repos.last().map(|s| s.is_empty()).unwrap_or(false) {
                    self.repos.pop();
                }
                return 0;
            }
            return -1;
        }
        return -1;
    }

    // 此方法检查仓库信息（来自远程的）是否存在。
    // 它首先检查远程目录是否存在，若不存在则创建该目录。
    // 然后遍历仓库名称列表，检查对应的信息文件是否存在。
    // 如果有任何文件缺失，会打印错误信息并返回-1；若所有文件都存在，则返回0。
    fn check_if_repoinfo_exist(&self) -> i32 {
        let remote_path = format!("{}/database/remote", self.config_dir);
        if !Path::new(&remote_path).exists() {
            std::fs::create_dir_all(&remote_path).unwrap();
        }

        for repo in &self.reponame {
            let infofile = format!("{}/database/remote/{}.json", self.config_dir, repo);
            if !Path::new(&infofile).exists() {
                println!(
                    "{}: error: repository index \"{}\" not found\nUse \"mcospkg-mirror",
                    std::env::args().next().unwrap(),
                    repo
                );
                return -1;
            }
        }
        return 0;
    }
}
