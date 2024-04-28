use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use racros::AutoDebug;

use crate::music::{Music, MusicState};

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

    /// Check whether contains contains the [`Music`] at `file_path`.
    pub fn contains(&self, file_path: &str) -> bool {
        self.music
            .iter()
            .find(|x| x.file_path == file_path)
            .is_some()
    }

    /// Get index (in playlist) of the previous music before the one at `file_path`.
    ///
    /// * Return `None` if `file_path` not exists in playlist.
    /// * Return the next music's index in playlist if found.
    /// * Return the last one if `file_path` is the first one.
    pub fn previous_of_path(&self, file_path: &str) -> Option<usize> {
        if self.music.is_empty() {
            return None;
        }
        let current = self.music.iter().position(|x| x.file_path == file_path);
        if current.is_none() {
            return None;
        }
        let current_index = current.unwrap();
        // Return the last one if is the first one.
        if current_index == 0 {
            return Some(self.music.len() - 1);
        }
        Some(current_index - 1)
    }

    /// Get index (in playlist) of the next music after the one at `file_path`.
    ///
    /// * Return `None` if `file_path` not exists in playlist.
    /// * Return the next music's index in playlist if found.
    /// * Return the first one if `file_path` is the last one.
    pub fn next_of_path(&self, file_path: &str) -> Option<usize> {
        if self.music.is_empty() {
            return None;
        }
        let current = self.music.iter().position(|x| x.file_path == file_path);
        if current.is_none() {
            return None;
        }
        let current_index = current.unwrap();
        // Return the first one if is the last one.
        if current_index >= self.music.len() {
            return Some(0);
        }
        Some(current_index + 1)
    }

    /// Get the music at `index`.
    ///
    /// * Return none if index out of range.
    pub fn music_at(&self, index: usize) -> Option<Music> {
        self.music.get(index).map(|x| x.to_owned())
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
    pub fn add_music_by_path(&mut self, path: &str) -> Result<usize> {
        self.traverse_dir_add_all(path)
    }

    fn add_file_to_list(&mut self, path: &str) -> Result<Music> {
        let pb = PathBuf::from(path);
        let file_name = pb.file_name().unwrap().to_str().unwrap().to_string();
        Ok(Music {
            file_path: path.to_string(),
            file_name,
            state: MusicState::Exists,
            metadata: None,
        })
    }

    fn traverse_dir_add_all(&mut self, path: &str) -> Result<usize> {
        let mut count = 0;
        let mut ret = vec![];
        let info = fs::metadata(path)?;
        if info.is_dir() {
            for entry in fs::read_dir(path)? {
                let metadata = entry.as_ref().unwrap().metadata()?;
                if metadata.is_dir() {
                    count += self.traverse_dir_add_all(path)?;
                }
                let file_name = entry
                    .as_ref()
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                ret.push(Music {
                    file_path: path.to_string(),
                    file_name,
                    state: MusicState::Exists,
                    metadata: None,
                });
                count += 1;
            }
        } else if info.is_file() && path.ends_with(".mp3") {
            ret.push(self.add_file_to_list(path)?);
        }

        self.add_music(ret);
        Ok(count)
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
