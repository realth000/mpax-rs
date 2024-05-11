use std::fs;
use std::io::{stdout, ErrorKind, Write};

use anyhow::{bail, Result};
use clap::{ArgAction, Args, Command, Parser, Subcommand};
use clap_complete::{generate, Shell};
use racros::AutoDebug;

use crate::cmd::pause::handle_pause_command;
use crate::cmd::play::handle_play_command;

mod pause;
mod play;

////////////// Args //////////////

#[derive(Args, AutoDebug, Clone)]
#[group(required = true, multiple = false)]
pub struct PlayTargetGroup {
    #[arg(short = 'n', long = "next", help = "play next song in current playlist", action = ArgAction::SetTrue)]
    pub next: Option<bool>,

    #[arg(short = 'p', long = "prev", help = "play previous song in current playlist", action = ArgAction::SetTrue)]
    pub prev: Option<bool>,

    #[arg(
        short = 'f',
        long = "file",
        help = "play specified music at given file path"
    )]
    pub file: Option<String>,
}

#[derive(Args, AutoDebug, Clone)]
pub struct PlayArgs {
    #[command(flatten)]
    pub play_target: PlayTargetGroup,
}

#[derive(Args, AutoDebug, Clone)]
pub struct PauseArgs {}

#[derive(AutoDebug, Clone, Parser)]
pub struct MpaxCtlCommand {
    #[command(subcommand)]
    pub command: Option<SubCommand>,

    #[arg(value_enum, long)]
    pub complete: Option<Shell>,
}

#[derive(AutoDebug, Clone, Subcommand)]
pub enum SubCommand {
    #[command(about = "Play music")]
    Play(PlayArgs),

    Pause(PauseArgs),

    Stop,

    Exit,
}

pub async fn run_command_with_args(command: MpaxCtlCommand) -> Result<()> {
    match command.command.unwrap() {
        SubCommand::Play(args) => handle_play_command(args).await?,
        SubCommand::Pause(args) => handle_pause_command(args).await?,
        SubCommand::Stop => unimplemented!(),
        SubCommand::Exit => unimplemented!(),
    }
    Ok(())
}

pub fn generate_completion(command: Command, generator: Shell) -> Result<()> {
    let mut save_path: Option<&str> = None;
    let mut save_target: Box<dyn Write> = match generator {
        Shell::Bash | Shell::Zsh => {
            save_path = Some(match generator {
                Shell::Bash => "/usr/share/bash-completion/completions/_mpaxctl",
                Shell::Zsh => "/usr/share/zsh/functions/Completion/Unix/_mpaxctl",
                _ => panic!("not going to happen"),
            });
            let file = match fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(save_path.unwrap())
            {
                Ok(v) => v,
                Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                    bail!(
                        "need root permission to save completion to {}",
                        save_path.unwrap()
                    )
                }
                Err(e) => return Err(e.into()),
            };

            Box::new(file)
        }
        _ => Box::new(stdout()),
    };
    generate(
        generator,
        &mut command.clone(),
        command.get_name().to_string(),
        &mut *save_target,
    );
    if let Some(v) = save_path {
        println!("shell completion saved to {v}");
    }
    Ok(())
}
