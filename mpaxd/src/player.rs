use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;

use anyhow::{anyhow, Context, Result};
use log::{debug, error, info};
use racros::AutoDebug;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rust_i18n::t;

/// Actions can apply to the player.
#[derive(AutoDebug)]
pub enum PlayAction {
    /// Play the given audio file with file path.
    Play(String),

    /// Pause the play process.
    Pause,

    /// Resume the play process.
    Resume,

    /// Stop the player.
    ///
    /// This will quit the player main loop.
    Stop,

    /// Turn to exit.
    ///
    /// Only do this operation when going to quit the daemon
    /// gracefully.
    Exit,
}

/// Defines all [Player] running modes.
#[derive(AutoDebug, PartialEq)]
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
#[derive(AutoDebug, Clone)]
pub enum AudioSource {
    /// An audio file, file path embedded.
    File(String),

    /// Network stream.
    ///
    /// **Not implemented**
    Network(String),
}

/// Source of audio for [Player] to play.
#[derive(AutoDebug)]
struct Audio {
    source: AudioSource,
    #[debug_value = "Decoder<File>"]
    decoder: Decoder<BufReader<File>>,
}

/// Status of [Player].
#[derive(AutoDebug, Clone, PartialEq)]
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
#[derive(AutoDebug)]
pub struct Player {
    /// Current status.
    status: PlayerStatus,

    /// The output stream on the audio device.
    #[debug_ignore]
    output_stream: OutputStream,

    /// Handler of [output_stream].
    #[debug_ignore]
    output_stream_handle: OutputStreamHandle,

    /// Sink to post [Audio] sources to audio devices.
    #[debug_ignore]
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

    /// Receiver to receive operations.
    rx: Receiver<PlayAction>,
}

/// FIXME: Check if is safe or use another way to achieve this.
unsafe impl Send for Player {}

/// Implementation of [Player].
impl Player {
    /// Construct a new instance of [Player].
    ///
    /// # Errors
    ///
    /// * When failed to sink the output device.
    pub fn new(rx: Receiver<PlayAction>) -> Result<Self> {
        let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).context(t!("player.failedToInit"))?;
        Ok(Player {
            status: PlayerStatus::Initial,
            output_stream,
            output_stream_handle,
            sink,
            audio: None,
            play_mode: PlayMode::RepeatPlaylist,
            rx,
        })
    }

    /// Play the audio file from given [path].
    ///
    /// # Errors
    ///
    /// * When failed to open [Audio] file path.
    /// * When failed to decode [Audio] file resource.
    pub async fn play_file(&mut self, path: &str) -> Result<()> {
        let file = BufReader::new(
            File::open(path).context(t!("player.canNotOpenAudioFile", path = path))?,
        );
        let source =
            Decoder::new(file).context(t!("player.failedToDecodeAudioFile", path = path))?;
        let (_s, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).context(t!("player.failedToInit"))?;
        sink.append(source);
        sink.sleep_until_end();

        self.sink.clear();
        // self.sink.append(source);
        self.status = PlayerStatus::Playing;
        self.sink.sleep_until_end();
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

    /// Run the player main loop.
    ///
    /// Waits for future operations sent from correspond tx and do actions.
    ///
    /// # Errors
    ///
    /// * When failed to receive [PlayAction] from [rx].
    pub async fn run_main_loop(&mut self) -> Result<()> {
        loop {
            let op = self
                .rx
                .recv()
                .context(anyhow!(t!("player.failedToReceiveOperation")))?;
            debug!("receive player action {op:#?}");
            match &op {
                PlayAction::Play(v) => {
                    if let Err(e) = self.play_file(v.as_str()).await {
                        error!("{e}");
                    } else {
                        debug!("start to play");
                    }
                }
                PlayAction::Pause => {
                    if let Err(e) = self.pause() {
                        error!("{e}")
                    } else {
                        debug!("paused");
                    }
                }
                PlayAction::Resume => self.resume(),
                PlayAction::Stop => self.stop(),
                PlayAction::Exit => {
                    // Stop the player and exit the app.
                    self.stop();
                    break;
                }
            }
        }
        println!(">????");
        Ok(())
    }
}

/// Launch and run the player thread
pub async fn launch_player_thread(rx: Receiver<PlayAction>) -> Result<()> {
    info!("player thread start");
    let mut player = Player::new(rx).context("player thread exit with error")?;
    player.run_main_loop().await?;
    info!("player thread exit");
    Ok(())
}
