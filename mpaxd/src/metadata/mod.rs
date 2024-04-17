use anyhow::Result;
use racros::AutoDebug;
use std::time::Duration;

/// Music metadata.
///
/// All fields have relative audio metadata tag in music file.
///
/// Now is representing id3v2 format.
#[derive(AutoDebug, Clone)]
pub struct Metadata {
    // Title.
    title: Option<String>,

    /// Artist name.
    ///
    /// Allow multiple artists.
    artist: Vec<String>,

    /// Album title.
    album: Option<String>,

    /// Duration.
    duration: Duration,
}

/// Definition the functionality of something that can done whe metadata related works.
trait MetadataParser {
    /// Parse a single metadata from given [`file_path`].
    fn parse_metadata_from_file(file_path: impl AsRef<str>) -> Result<Metadata>;
}
