use crate::metadata::Metadata;
use racros::{AutoDebug, AutoStr};

/// Enum indicating music's status.
#[derive(AutoDebug, AutoStr, Clone, Default, PartialEq)]
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
    file_path: String,

    /// File name of the music file.
    file_name: String,

    /// State, file exists or not.
    ///
    /// Default if [`MusicState::Unknown`] before first check.
    state: MusicState,

    /// Metadata of the file.
    ///
    /// Cached value in memory or database..
    metadata: Option<Metadata>,
}
