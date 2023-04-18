use clap::{Parser, Subcommand};
use crate::init::InitArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opts {
    #[command(subcommand)]
    pub sub: SubCommands,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum SubCommands {
    /// Initialize a new xnft project
    Init(InitArgs),
}
