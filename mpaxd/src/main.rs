#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
use std::error::Error;
use std::io::{Read, Seek};
use std::sync::mpsc::channel;

use anyhow::Result;
use futures::task::SpawnExt;
use rust_i18n::i18n;

use crate::player::{launch_player_thread, PlayAction};
use crate::server::launch_server_thread;

i18n!("i18n");

mod config;
mod metadata;
mod music;
mod player;
mod playlist;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let (tx, rx) = channel::<PlayAction>();
    let _tx = tx.clone();
    let player_thread_handle = tokio::spawn(launch_player_thread(_tx, rx));
    let server_thread_handle = tokio::spawn(launch_server_thread(tx));
    player_thread_handle.await??;
    server_thread_handle.await??;
    //join_all([player_thread_handle, server_thread_handle]).await;
    Ok(())
}
