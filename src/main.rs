mod cli;
mod core;
mod modules;
mod services;

use clap::Parser;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::CliArgs::parse();

    match args.command {
        cli::Commands::Git(args) => modules::git::handle(args),
        cli::Commands::Scan(args) => modules::docker::scan::handle(args),
        cli::Commands::Docker(args) => modules::docker::handle(args),
        cli::Commands::Auth(args) => services::auth::handle(args),
        cli::Commands::Log(args) => modules::git::log::handle(&args),
        cli::Commands::Http(args) => modules::http::handler::handle(args.action).await?,
    }

    Ok(())
}
