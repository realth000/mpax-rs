use std::sync::mpsc::Sender;

use anyhow::Result;
use log::info;

use crate::player::PlayAction;

/// Launch a thread for the socket to run on.
///
/// Listen for the messages from client (mpaxctl) and act to it.
///
/// This function takes a [Sender] type argument [tx] to send
/// Operations to the [Player].
pub async fn launch_socket_thread(tx: Sender<PlayAction>) -> Result<()> {
    info!("socket thread start");
    // TODO: Launch the socket server.
    // TODO: Convert client requests into [PlayAction]s.

    //listen_and_
    // tokio::time::sleep(Duration::from_secs(100)).await;

    info!("socket thread exit");
    Ok(())
}
