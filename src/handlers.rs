use crate::args::CheckCommand;
use anyhow::Result;
use clap::crate_version;

pub fn check_command(checkcmd: CheckCommand) -> Result<()> {
    println!("INFO: check command: checkcmd: {:?}", checkcmd);

    Ok(())
}

pub fn env_command() -> Result<()> {
    println!("INFO: env command");

    Ok(())
}

pub fn version_command() -> Result<()> {
    println!("INFO: version command: version: {:?}", crate_version!());

    Ok(())
}
