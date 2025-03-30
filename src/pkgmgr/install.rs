use crate::Package;
use indicatif::{ProgressBar, ProgressStyle};

fn step1(pb: &mut ProgressBar) {
    for _ in 30..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

pub fn install_pkg(packages: Vec<Package>) -> i32 {
    // Iterate the index and set the ProgressBar
    for package in packages {
        let mut pb = ProgressBar::new(100);
        let style = ProgressStyle::default_bar()
            .template("{msg} {eta_precise} [{bar:40.green/blue}] {percent}%")
            .unwrap()
            .progress_chars("##-");
        pb.set_style(style);
        let package_msg: std::borrow::Cow<'static, str> = package.id.into();
        pb.set_message(package_msg);

        // Simulate incresing
        for _ in 0..30 {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
        step1(&mut pb);
        pb.finish();
    }
    
    0   // return temply
}
