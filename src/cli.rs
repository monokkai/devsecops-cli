use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "monokkai")]
#[command(about = "DevSecOps CLI Tool", version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Git operations
    Git(GitArgs),
    /// Security scanning
    Scan(ScanArgs),
    /// Docker operations
    Docker(DockerArgs),
    /// Authentication
    Auth(AuthArgs),
}

#[derive(Parser)]
pub struct GitArgs {
    #[arg(short, long)]
    pub message: String,

    #[arg(short, long, default_value_t = false)]
    pub push: bool,
}

#[derive(Parser)]
pub struct ScanArgs {
    #[arg(short, long)]
    pub path: String,
}

#[derive(Parser)]
pub struct DockerArgs {
    #[command(subcommand)]
    pub action: DockerAction,
}

#[derive(Subcommand)]
pub enum DockerAction {
    Scan { image: String },
    Push { image: String, tag: Option<String> },
}

#[derive(Parser)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand)]
pub enum AuthAction {
    Jwt { token: String },
    Github { token: String },
}
