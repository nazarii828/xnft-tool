use std::borrow::Borrow;

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
        /// generate template with react native
        #[arg(long)]
        rn: bool,
    }, // ! a user should also be able to use new
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name, rn } => {
            //generate react-native if rn flag is passed
            if *rn {
                if name.is_none() {
                    let name = "xnft-project".to_string();
                    Create::new_rn(&name).unwrap();
                    return;
                }
                Create::new_rn(&name.to_owned().unwrap()).unwrap();
            } else {
                // default. generate without react-native
				if name.is_none() {
                    let name = "xnft-project".to_string();
                    Create::new_native(&name).unwrap();
                    return;
                }
                Create::new_native(&name.to_owned().unwrap()).unwrap();
            }
        }
        _ => println!("command not recognized"),
    }
}
