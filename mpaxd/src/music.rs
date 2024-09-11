use racros::{AutoDebug, AutoStr};

use crate::metadata::Metadata;

/// Enum indicating music's status.
#[derive(AutoDebug, AutoStr, Clone, Default, PartialEq, Eq)]
pub enum MusicState {
    /// Music file exists.
    Exists,

    /// Music file not exists.
    NotExists,

    /// Unknown state or no checked.
    #[default]
    Unknown,
}

#[derive(AutoDebug, Clone)]
pub struct Music {
    /// File path of the music file.
    pub file_path: String,

    /// File name of the music file.
    pub file_name: String,

    /// State, file exists or not.
    ///
    /// Default if [`MusicState::Unknown`] before first check.
    pub state: MusicState,

    /// Metadata of the file.
    ///
    /// Cached value in memory or database.
    pub metadata: Option<Metadata>,
}
