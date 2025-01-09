// Well, this usually in use to print the information of the
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
    #[arg(required = false, help = "Specify a Information type, Support: os-license, repo-site, all", default_value_t = String::from("all"))]
    info_type: String,  // The info type, it's only a string
}

// Fourth, let's do it!
fn main() {
    // Define the colorful text
    let error = "error".red().bold();

    // Parse args
    let args = Args::parse();

    // And, let's check the condition:
    match args.info_type.as_str() {
        "all" => println!("{}\n\n{} in this program, Repository URLS is {};\n\nExecutable files are: \n\tmcospkg, \n\tmcospkg-mirror, \n\tmcospkg-info\n\n", introduce(), os_license(), repo_site()),
        "os-license" => println!("{}", os_license()),
        "repo-site" => println!("{}",repo_site()),
        "usage" => println!(""),
        "introduce" => println!("{}", introduce()),
        &_ => println!("{}: unknown options: {}\n{}: Supports arguments are: os-license, repo-site, all", error, args.info_type, "tip".bold().green()),
    }
}

fn introduce() -> String {
    String::from("Mcospkg, producted by a 13-year-old boy.")
}

fn os_license() -> String {
    String::from("Uses license GPL-3.0")
}

fn repo_site() -> String {
    String::from("https://github.com/zhangxuan2011/mcospkg")
}
