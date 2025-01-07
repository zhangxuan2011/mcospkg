use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

pub fn download(url: String, save: String, msg: &'static str) -> Result<(), Error> {
    let mut resp =
        get(url).map_err(|e| Error::new(ErrorKind::Other, format!("Cannot fetch file: {}", e)))?;
    let mut file = File::create(save)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Cannot create file: {}", e)))?;

    let pb = ProgressBar::new(resp.content_length().unwrap_or(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - {msg}\n\n",
            )
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_message(&*msg);

    let mut downloaded_bytes = 0;
    let mut buffer = [0; 8192];
    loop {
        match resp.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                file.write_all(&buffer[0..bytes_read])
                    .map_err(|e| Error::new(ErrorKind::Other, format!("写入文件出错: {}", e)))?;
                downloaded_bytes += bytes_read;
                pb.set_position(downloaded_bytes as u64);
            }
            _ => break,
        }
    }
    pb.finish();
    Ok(())
}
