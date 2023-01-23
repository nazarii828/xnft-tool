use clap::{Parser, Subcommand};

use crate::commands::{Create, Template};

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
    },

    /// Use an xNFT template
    Template {
        /// list available templates
        #[arg(short, long)]
        list: bool,

        /// use the default xnft-quickstart template
        #[arg(short, long)]
        default: bool,

        /// specify a template to use
        #[arg(short, long)]
        get: Option<String>,
    },
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
        Commands::Template { list, default, get } => {
            if *list {
                Template::print_available_templates();
            };
            if *default {
                Template::default_template();
            };
            if get.is_some() {
                Template::get_template(get.to_owned().unwrap().as_str());
            }
        }
        _ => println!("command not recognized"),
    }
}
