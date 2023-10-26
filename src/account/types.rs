use serde::Deserialize;
use std::collections::HashMap;

pub struct UnregisterDeviceOptions {
    pub sandbox: bool,
}

impl Default for UnregisterDeviceOptions {
    fn default() -> Self {
        Self { sandbox: false }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResponseInfo {
    #[serde(alias = "2fa_required")]
    pub twofa_required: Option<u8>,
    pub country: Option<String>,
    pub https_required: Option<u8>,
    pub intro: Option<u8>,
    pub community_comments: Option<bool>,
    pub link_redirects: Option<HashMap<String, String>>,
    pub lang: Option<u8>,
    pub no_wall_replies: Option<u8>,
    pub own_posts_default: Option<u8>,
    pub vk_pay_endpoint_v2: Option<String>,
    pub messages_translation_language_pairs: Option<Vec<String>>,
    pub obscene_text_filter: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResponseBanned {
    pub count: u32,
    pub items: Vec<u32>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub can_access_closed: bool,
    pub is_closed: bool,
}

pub struct SetOnlineOptions {
    pub voip: bool,
}

impl Default for SetOnlineOptions {
    fn default() -> Self {
        Self { voip: false }
    }
}

pub struct GetInfoOptions {
    pub country: bool,
    pub https_required: bool,
    pub own_posts_default: bool,
    pub no_wall_replies: bool,
    pub intro: bool,
    pub lang: bool,
}

impl Default for GetInfoOptions {
    fn default() -> Self {
        Self {
            country: false,
            https_required: false,
            own_posts_default: false,
            no_wall_replies: false,
            intro: false,
            lang: false,
        }
    }
}
