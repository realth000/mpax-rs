use anyhow::Result;
use clap::{CommandFactory, Parser};
use reqwest::redirect::Policy;
use reqwest::Client;

use self::cmd::{generate_completion, run_command_with_args, MpaxCtlCommand};

mod client;
mod cmd;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
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
