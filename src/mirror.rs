// First, import some modules we need
mod config;
mod mirror {
    pub mod update;
    pub mod add;
}
use clap::{Parser, Subcommand};
use config::VERSION;
use mirror::update::UpdateData;
use mirror::add::AddData;

// And then we define the arguments
#[derive(Parser, Debug)]
#[command(name = "mcospkg-mirror")]
#[command(about = "The mirror list manager of mcospkg")]
#[command(version = VERSION)]
struct Args {
    #[command(subcommand)]
    operation: Operations,
}

#[derive(Subcommand, Debug)]
enum Operations {
    #[command(about = "Update the mirror list")]
    Update,

    #[command(about = "Add a mirror")]
    Add {
        #[arg(help = "The name of the repository")]
        reponame: String,

        #[arg(help = "The url of the repository")]
        repourl: String,
    },

    #[command(about = "Delete a mirror")]
    Delete {
        #[arg(help = "The name of the repository")]
        reponame: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.operation {
        Operations::Update => update(),
        Operations::Add { reponame, repourl } => {
            add(reponame, repourl);
        }
        Operations::Delete { reponame } => {
            delete(reponame);
        }
    }
}

fn update() {
    // Initialize the data
    let mut update_data = UpdateData::new();

    // Then do the steps
    update_data.step1_readcfg();

    update_data.step2_fill_msgs();

    update_data.step3_create_dirs();

    update_data.step4_download_index();

    // And completed!
}

fn add(reponame: String, repourl: String) {
    // Initialize it
    let mut add_data = AddData::new();

    // Then do a step
    let _ = add_data.step_matches(reponame, repourl);

    // And...Done
}

fn delete(_reponame: String) {}
