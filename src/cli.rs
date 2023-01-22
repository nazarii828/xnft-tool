use clap::Arg;
use clap::{Command, Parser, Subcommand};

use crate::utils::Create;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new empty xnft-project
    Init {
        /// name of your project
        name: Option<String>,
    }, // ! a user should also be able to use new
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => {
            if name.is_none() {
                let name = "xnft-project".to_string();
                Create::new(&name).unwrap();
                return;
            }
            Create::new(&name.to_owned().unwrap()).unwrap();
        },
        _ => println!("command not recognized"),
    }
}
