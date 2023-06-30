

#[derive(Debug)]
pub enum MusicPlayerAction {
    Play,
    Pause,
    Seek(i32),
    Next,
    Previous,

}
