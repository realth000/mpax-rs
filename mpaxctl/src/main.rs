#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
use anyhow::Result;
use clap::{CommandFactory, Parser};

use self::cmd::{generate_completion, run_command_with_args, MpaxCtlCommand};

mod client;
mod cmd;
mod config;
mod url;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let command = MpaxCtlCommand::parse();
    if let Some(shell) = command.complete {
        return generate_completion(MpaxCtlCommand::command(), shell);
    }

    if command.command.is_none() {
        MpaxCtlCommand::command().print_help()?;
        return Ok(());
    }

    run_command_with_args(command).await
}
