use std::io::{Read, Seek};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use anyhow::Result;
use rust_i18n::i18n;

use crate::player::{PlayAction, Player};

i18n!("mpaxd/i18n");

mod player;

/// Launch and run the player thread
async fn launch_player_thread(rx: Receiver<PlayAction>) -> Result<()> {
    let mut player = Player::new(rx)?;
    player.run_main_loop().await?;
    println!("player thread exit");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let (tx, rx) = channel::<PlayAction>();
    let play_thread_handle = tokio::spawn(launch_player_thread(rx));
    tokio::time::sleep(Duration::from_secs(2)).await;
    tokio::join!(play_thread_handle);
    Ok(())
}
