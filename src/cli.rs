use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "monokkai")]
#[command(about = "DevSecOps CLI Tool", version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Git(GitArgs),
    Scan(ScanArgs),
    Docker(DockerArgs),
    Auth(AuthArgs),
    Log(LogArgs),
    Http(HttpArgs),
}

#[derive(Parser)]
pub struct GitArgs {
    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub push: bool,

    #[arg(short, long, default_value_t = false)]
    pub add: bool,

    #[arg(short, long, default_value_t = false)]
    pub pull: bool,

    #[arg(short, long, default_value_t = false)]
    pub rebase: bool,

    #[command(subcommand)]
    pub subcommand: Option<GitSubcommand>,
}

#[derive(Subcommand)]
pub enum GitSubcommand {
    Cz {
        #[arg(short, long, default_value_t = false)]
        add: bool,

        #[arg(short, long, default_value_t = false)]
        push: bool,

        #[arg(short, long, default_value_t = false)]
        pull: bool,

        #[arg(short, long, default_value_t = false)]
        rebase: bool,
    },
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

#[derive(Parser)]
pub struct LogArgs {
    #[arg(short, long, default_value_t = false)]
    pub compact: bool,

    #[arg(short, long)]
    pub limit: Option<usize>,

    #[arg(short, long, default_value_t = false)]
    pub graph: bool,
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

#[derive(Parser)]
pub struct HttpArgs {
    #[command(subcommand)]
    pub action: HttpAction,
}

#[derive(Subcommand)]
pub enum HttpAction {
    Get {
        url: String,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Post {
        url: String,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Put {
        url: String,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Patch {
        url: String,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Delete {
        url: String,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Head {
        url: String,
        #[arg(short, long)]
        headers: Vec<String>,
    },
    Options {
        url: String,
        #[arg(short, long)]
        headers: Vec<String>,
    },
}
