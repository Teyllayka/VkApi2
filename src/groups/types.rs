use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub edit_title: Option<u8>,
    pub desc: String,
    pub image_processing: Option<u8>,
}

pub struct JoinOptions {
    pub not_sure: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetOnlineStatusResponse {
    pub status: OnlineStatus,
    pub minutes: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnlineStatus {
    None,
    Online,
    AnswerMark,
}

impl Default for JoinOptions {
    fn default() -> Self {
        Self { not_sure: false }
    }
}
