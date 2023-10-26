use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GiftsResult {
    pub count: usize,
    pub items: Vec<Gift>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Gift {
    pub id: usize,
    pub from_id: usize,
    pub message: String,
    pub date: usize,
    pub gift: GiftInfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GiftInfo {
    pub id: usize,
    pub thumb_256: String,
    pub thumb_96: String,
    pub thumb_48: String,
}
