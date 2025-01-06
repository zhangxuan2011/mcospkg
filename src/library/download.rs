use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;

pub fn download(url: String, save: String) {
    let mut response = get(url).expect("Cannot fetch file");
    let mut file = File::create(&save.as_str()).expect("Cannot create file");

    // 创建进度条并设置描述信息
    let pb = ProgressBar::new(response.content_length().unwrap_or(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
    );

    // 下载文件并更新进度条
    let mut downloaded_bytes = 0;
    copy(&mut response, &mut file).expect("Cannot copy file content");
    pb.finish();
}
