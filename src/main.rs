mod cli;
mod commands;
// mod utils;

use crate::cli::{CliArgs, Commands};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Git(args) => commands::git::handle(args),
        Commands::Scan(args) => commands::scan::handle(args),
        Commands::Docker(args) => commands::docker::handle(args),
        Commands::Auth(args) => commands::auth::handle(args),
    }
}
