pub mod music_player;

pub use music_player::*;


#[derive(Debug)]
pub enum UiToCore {
    Exit {
        user_initiated: bool,
    },
    MusicPlayerAction(MusicPlayerAction),
    MusicPlayerSetting(),
}
