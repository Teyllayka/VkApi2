use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusGetResponse {
    pub text: String,
    pub audio: Option<Audio>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub artist: String,
    pub id: u64,
    pub owner_id: u64,
    pub title: String,
    pub duration: u64,
    pub is_explicit: bool,
    pub is_focus_track: bool,
    pub track_code: String,
    pub url: String,
    pub date: u64,
    pub genre_id: u64,
    pub short_videos_allowed: bool,
    pub stories_allowed: bool,
    pub stories_cover_allowed: bool,
}
