use clap::Parser;
use commands::{git, scan, docker, auth};

mod commands;
mod utils;
mod cli;

fn main() {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Git(git_args) => git::handle(git_args),
        cli::Commands::Scan(scan_args) => scan::handle(scan_args),
        cli::Commands::Docker(docker_args) => docker::handle(docker_args),
        cli::Commands::Auth(auth_args) => auth::handle(auth_args),
    }
}