// Now, we need to import some modules:
use clap::Parser;   // Argument parser

// Configure parser
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.0-debug")]

// Define argument lists
struct Args {
    #[arg(required = true, help = "Supports: install/remove/update")]
    options: String,
    
    #[arg(required = false)]
    packages: Vec<String>,

    #[arg(long = "bypass", short = 'y', default_value_t = false, help = "Specify it will not ask ANY questions")]
    bypass_ask: bool,
}

fn main() {
    let args = Args::parse();
    match args.options.as_str() {
        "install" => install(args.packages),
        "remove" => remove(args.packages),
        _ => todo!(),
    };    
}

fn install(pkgindex: Vec<String>) {}

fn remove(pkgindex: Vec<String>) {}
