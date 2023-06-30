use arc_swap::ArcSwap;

use super::config::MainConfig;
use super::audio::MusicPlayer;


/// The main state of the application.
/// This is shared between the UI and the background thread (the "core").
#[derive(Debug, Default)]
pub struct MainState {
    pub config: ArcSwap<MainConfig>,

    pub music_player: MusicPlayer,
}
