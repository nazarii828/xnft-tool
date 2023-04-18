use clap::Parser;
use cli::Opts;

mod cli;
mod init;
mod utils;

use cli::SubCommands;
use utils::Cmd;// must be in scope to be used 

fn main() {
    let opts = Opts::parse();
    match opts.sub {
        SubCommands::Init(cmd) => {
            cmd.run().unwrap();
        }
    }
}
