use std::fs::File;
use std::io::BufReader;

use anyhow::{anyhow, Context, Result};
use rodio::{Decoder, OutputStream, Sink};
use rust_i18n::t;

/// Defines all [Player] running modes.
#[derive(PartialEq)]
enum PlayMode {
    /// Repeat to play the current playlist.
    RepeatPlaylist,
    /// Repeat to play the current [Audio].
    RepeatSingle,
    /// Randomly play [Audio]s in playlist.
    Random,
}

/// Source type of [Audio]s.
///
/// Where this [Audio] came from.
#[derive(Clone)]
pub enum AudioSource {
    /// An audio file, file path embedded.
    File(String),
    /// Network stream.
    ///
    /// **Not implemented**
    Network(String),
}

/// Source of audio for [Player] to play.
struct Audio {
    source: AudioSource,
    decoder: Decoder<BufReader<File>>,
}

/// Status of [Player].
#[derive(Clone, PartialEq)]
enum PlayerStatus {
    /// Initializing.
    Initial,
    /// Playing audio
    Playing,
    /// Playing audio but paused.
    Paused,
    /// Stopped and waiting for play next one.
    Stopped,
}

/// The player to handle playing audio tasks.
pub struct Player {
    /// Current status.
    status: PlayerStatus,
    /// Sink to post [Audio] sources to audio devices.
    sink: Sink,
    /// [Audio] resource current playing.
    ///
    /// [None] represents not holding any resource.
    ///
    /// Actually this field should be held in [PlayerStatus].
    /// But since every time switching the player status will
    /// cause copy-paste-deconstruct, use a separate field to hold it.
    audio: Option<Audio>,
    /// Player running mode, decide the action when current playing
    /// [Audio] finished.
    play_mode: PlayMode,
}

/// Implementation of [Player].
impl Player {
    /// Construct a new instance of [Player].
    ///
    /// # Errors
    ///
    /// * When failed to sink the output device.
    pub fn new() -> Result<Self> {
        let (_s, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).context(t!("player.failedToInit"))?;
        Ok(Player {
            status: PlayerStatus::Initial,
            sink,
            audio: None,
            play_mode: PlayMode::RepeatPlaylist,
        })
    }

    /// Play the audio file from given [path].
    ///
    /// # Errors
    ///
    /// * When failed to open [Audio] file path.
    /// * When failed to decode [Audio] file resource.
    pub fn play_file(&mut self, path: &str) -> Result<()> {
        let file = BufReader::new(
            std::fs::File::open(path).context(t!("player.canNotOpenAudioFile", path = path))?,
        );
        let source =
            Decoder::new(file).context(t!("player.failedToDecodeAudioFile", path = path))?;
        self.sink.clear();
        self.sink.append(source);
        self.status = PlayerStatus::Playing;
        Ok(())
    }

    /// Pause the player, keep holding [Audio] resources.
    ///
    /// # Errors
    ///
    /// * When player is not holding any [Audio] resource.
    pub fn pause(&mut self) -> Result<()> {
        if self.audio.is_none() {
            return Err(anyhow!(t!("player.canNotPauseNoAudioLoaded")));
        }
        match &self.status {
            PlayerStatus::Initial | PlayerStatus::Paused | PlayerStatus::Stopped => {
                // Do nothing.
            }
            PlayerStatus::Playing => {
                self.status = PlayerStatus::Paused;
                self.sink.pause();
            }
        }
        Ok(())
    }

    /// Resume to play.
    ///
    /// Do nothing if not in [PlayerStatus::Paused] state.
    pub fn resume(&mut self) {
        if self.status == PlayerStatus::Paused {
            self.sink.play();
            self.status = PlayerStatus::Playing;
        }
    }

    /// Stop the player and release the held [Audio] resources.
    ///
    /// Do nothing if already in [PlayerStatus::Stopped] state.
    pub fn stop(&mut self) {
        if self.status != PlayerStatus::Stopped {
            self.sink.stop();
            self.audio = None;
            self.status = PlayerStatus::Stopped;
        }
    }

    pub async fn run_main_loop(&self) {
        loop {}
    }
}
