use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mcospkg(mirror)")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = "0.1.0-debug")]

struct Args {
    #[arg(required = true)]
    option: String,
}

fn main() {
    let args = Args::parse();
    match args {
        _ => todo!(),
    }
}
