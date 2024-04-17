use crate::music::Music;
use racros::AutoDebug;

#[derive(AutoDebug, Clone)]
pub struct Playlist {
    /// Name of the playlist.
    name: String,

    /// All [`Music`] in playlist.
    music: Vec<Music>,
}

impl Playlist {
    /// Construct.
    pub fn new(name: String) -> Playlist {
        Playlist {
            name,
            music: vec![],
        }
    }

    /// Get the name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Add music.
    pub fn add_music(&mut self, music: Vec<Music>) {
        self.music.extend(music);
    }

    /// Add music by specify the file path or folder path.
    ///
    /// * If [`path`] is a file, add it.
    /// * If [`path`] is a directory, add all music under it (include subdirectory).
    /// * If [`path`] not exists, do nothing.
    ///
    /// Return the number of [`Music`] successfully added.
    pub fn add_music_by_path(&mut self, path: impl AsRef<str>) -> usize {
        unimplemented!()
    }

    /// Remove music by specify the file path or folder path.
    ///
    /// Only remove from playlist, not delete from disk.
    ///
    /// * If [`path`] is a file, remove it.
    /// * If [`path`] is a directory, add all the music under it (include subdirectory) if in
    ///   playlist.
    /// * If [`path`] not exists, do nothing.
    ///
    /// Return the number of [`Music`] removed.
    pub fn remove_music_by_path(&mut self, path: impl AsRef<str>) -> usize {
        unimplemented!()
    }
}
