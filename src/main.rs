// This only be used in linux platform, so we difine it:
#[cfg(target_os = "linux")]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = "0.1.0-debug")]

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
    println!("mcospkg {} {:#?} , bypass={}", args.options, args.packages, args.bypass_ask);
}
