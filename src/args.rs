use crate::handlers;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about = "Kata Containers control tool")]
pub struct AppArgs {
    #[clap(subcommand)]
    pub arguments: Arguments,

    #[clap(short, long)]
    pub log_level: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Arguments {
    /// Tests if system can run Kata Containers
    Check(CheckCommand),

    /// Display settings
    Env,

    /// Display version details
    Version,
}

#[derive(Debug, Args)]
pub struct CheckCommand {
    #[clap(subcommand)]
    pub command: CheckSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum CheckSubCommand {
    /// Runs all checks
    All,

    /// Runs all checks but excluding network checks.
    NoNetworkChecks,

    /// Only compare the current and latest available versions
    CheckVersionOnly,
}

pub fn handle() -> Result<()> {
    let args = AppArgs::parse();

    let log_level = args.log_level.unwrap_or("info".to_string());

    println!("INFO: log level: {:?}", log_level);

    match args.arguments {
        Arguments::Check(checkcmd) => handlers::check_command(checkcmd),
        Arguments::Env => handlers::env_command(),
        Arguments::Version => handlers::version_command(),
    }
}
