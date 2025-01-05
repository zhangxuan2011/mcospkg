use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = "0.1.0-debug")]

struct Args {
    #[arg(required = true, help = "update/add/delete are the avainable option")]
    option: String,
}

fn main() {
    let args = Args::parse();
    match args.option.as_str() {
        "update" => update(),
        "add" => add(),
        "delete" => delete(),
        _ => todo!(),
    }
}

fn update() {
    
}

fn add() {}

fn delete() {}
