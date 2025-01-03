// Well, this usually use to print the information of the
// mcospkg, and its usage.
//
// This supports to give you a lot of ibformation type, and 
// you just need to add a info-type after the mcospkg-info.
//
// Ok, no more bullshit, let's start!

// First, we need to import some modules:
use clap::Parser;   // To parse argument(info-type)
use colored::Colorize;  // To show colorful text(in var "error")

// And, set up the parser information:
#[derive(Parser, Debug)]
#[command(name = "mcospkg-info")]
#[command(about = "Information of mcospkg")]
#[command(version = "0.1.0-debug")] // The version is same as the main's.

// Third, we need to define a struct, it shows the argument options.
struct Args {
    #[arg(required = false, help = "Specify a Information type, Support: os-license, repo-site, default", default_value_t = String::from("default"))]
    info_type: String,  // The info type, its only a string
}

// Fourth, let's do it!
fn main() {
    // Define the colorful text
    let error = "error".red();

    // Parse args
    let args = Args::parse();

    // And, let's check the condition:
    match args.info_type.as_str() {
        "default" => println!("Mcospkg, producted by a 13-year-old boy.\n\nThis program uses license GPL-3.0, Repository URLS is https://github.com/zhangxuan2011/mcospkg; Executable files are: \n\tmcospkg, \n\tmcospkg-mirror, \n\tmcospkg-info\n\nUsage:...."),
        "os-license" => println!("License uses GPL-3.0"),
        "repo-site" => println!("https://github.com/zhangxuan2011/mcospkg"),
        &_ => println!("{}: unknown options", error),
    }
}
