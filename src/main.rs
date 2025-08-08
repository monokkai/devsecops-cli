mod cli;
mod modules;
mod services;

use crate::cli::{CliArgs, Commands};
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Git(args) => modules::git::git::handle(args),
        Commands::Scan(args) => modules::docker::scan::handle(args),
        Commands::Docker(args) => modules::docker::docker::handle(args),
        Commands::Auth(args) => services::auth::handle(args),
        Commands::Log(args) => modules::git::log::handle(&args),
    }
}
