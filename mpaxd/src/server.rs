use std::sync::mpsc::Sender;

use anyhow::Result;
use axum::Router;
use axum::routing::get;
use log::info;
use tokio::net::TcpListener;

use crate::player::PlayAction;

/// Launch a thread for the socket to run on.
///
/// Listen for the messages from client (mpaxctl) and act to it.
///
/// This function takes a [Sender] type argument [tx] to send
/// Operations to the [Player].
pub async fn launch_server_thread(tx: Sender<PlayAction>) -> Result<()> {
    info!("server thread start");
    let server = Router::new().route("/", get(|| async { "Hello World!" }));
    let listener = TcpListener::bind("0.0.0.0:18519").await.unwrap();
    axum::serve(listener, server).await.unwrap();
    info!("server thread exit");
    Ok(())
}
