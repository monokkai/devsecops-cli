mod cli;
mod commands;
mod utils;

use std::env::Args;
use clap::Parser;

use crate::cli::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Git(args) => commands::git::handle(args),
        Commands::Scan(args) => commands::scan::handle(args),
        Commands::Docker(args) => commands::docker::handle(args),
        Commands::Auth(args) => commands::auth::handle(args),
    }
}
