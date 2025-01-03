use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mcospkg-info")]
#[command(about = "Information of mcospkg")]
#[command(version = "0.1.0-debug")]

struct Args {
    #[arg(required = false, help = "Specify a Information type, Support: os-license, repo-site, default", default_value_t = String::from("default"))]
    info_type: String,
}

fn main() {
    let args = Args::parse();
    match args.info_type.as_str() {
        "default" => println!("Mcospkg, producted by a 13-year-old boy.\n\nThis program uses license GPL-3.0, Repository URLS is https://github.com/zhangxuan2011/mcospkg; Executable files are: \n\tmcospkg, \n\tmcospkg-mirror, \n\tmcospkg-info\n\nUsage:...."),
        "os-license" => println!("License uses GPL-3.0 ."),
        "repo-site" => println!("https://github.com/zhangxuan2011/mcospkg"),
        &_ => todo!(),
    }
}
