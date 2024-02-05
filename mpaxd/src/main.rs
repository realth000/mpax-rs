use std::io::{Read, Seek};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use rust_i18n::i18n;

use crate::player::Player;

i18n!("mpaxd/i18n");

mod player;

// "/home/lth/test.mp3"

/// Launch and run the player thread
async fn launch_player_thread(player: &mut Mutex<Player>) {
    let _x = player.lock().unwrap().run_main_loop();
    println!("player thread exit");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut player = Arc::new(Mutex::new(Player::new()?));
    let mut player_clone = player.clone();
    let play_thread_handle = tokio::spawn(launch_player_thread(&mut player_clone));
    tokio::join!(play_thread_handle);
    Ok(())
}
