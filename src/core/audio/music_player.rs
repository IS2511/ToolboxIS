

// TODO: Use Symphonia
#[derive(Debug)]
pub struct MusicPlayer {
    songs: Vec<Song>,
}

impl MusicPlayer {
    pub fn new_with_empty_queue() -> Self {
        Self {
            songs: Vec::new(),
        }
    }
}


#[derive(Debug)]
pub struct Song {
    cover: Option<SongCover>,
    lyrics: Option<SongLyrics>,
}


#[derive(Debug)]
pub struct SongCover {

}

#[derive(Debug)]
pub struct SongLyrics {

}
