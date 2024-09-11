use racros::AutoDebug;
use serde::{Deserialize, Serialize};

/// Operations defined for clients to use in request.
#[derive(AutoDebug, Deserialize, Hash, Serialize, PartialEq, Eq)]
pub enum RequestAction {
    /// Request to play an audio file with file path.
    Play(String),

    /// Request to pause the player.
    Pause,

    /// Request to resume the play process.
    Resume,

    /// Request to stop the player.
    Stop,

    /// Request to stop the player and let the server exits.
    Exit,
}

/// Model used for client to send request to the server side.
#[derive(AutoDebug, Deserialize, Hash, Serialize, PartialEq, Eq)]
pub struct AudioRequestModel {
    action: RequestAction,
}
