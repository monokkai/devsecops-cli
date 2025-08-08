use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devsecops-cli")]
#[command(
    about = "DevSecOps CLI: аудит безопасности, упрощённый git и др.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Git commands(easier than GitHub cli)
    Git(GitArgs),

    // Scan code
    Scan(ScanArgs),

    // Audit
    Docker(DockerArgs),

    // Jwt, OAuth, auth..
    Auth(AuthArgs),
}

#[derive(Parser)]
pub struct GitArgs {
    #[arg(short, long)]
    message: String,

    #[arg(short, long, default_value = "false")]
    push: bool,
}

#[derive(Parser)]
pub struct ScanArgs {
    #[arg(short, long)]
    path: String,
}

#[derive(Parser)]
pub struct DockerArgs {
    #[command(subcommand)]
    pub action: DockerAction,
}

#[derive(Subcommand)]
pub enum DockerAction {
    Scan {
        #[arg(short, long)]
        image: String,
    },
    Audit {
        #[arg(short, long, default_value = "false")]
        verbose: bool,
    },
}

#[derive(Parser)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand)]
pub enum AuthAction {
    Jwt {
        #[arg(short, long)]
        token: String,
    },
    Oauth {
        #[arg(short, long)]
        token: String,
    },
}
