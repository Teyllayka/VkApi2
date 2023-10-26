use serde::{Deserialize, Serialize};

pub enum AddType {
    Post,
    Comment,
    Photo,
    Audio,
    Video,
    Note,
    Market,
    PhotoComment,
    VideoComment,
    TopicComment,
    MarketComment,
}

pub enum DeleteType {
    Post,
    Story,
    Comment,
    Photo,
    Audio,
    Video,
    Note,
    Market,
    PhotoComment,
    VideoComment,
    TopicComment,
    MarketComment,
    SitePage,
}

pub enum GetList {
    Post,
    PostAds,
    Comment,
    Photo,
    Audio,
    Video,
    Note,
    Market,
    PhotoComment,
    VideoComment,
    TopicComment,
    MarketComment,
    SitePage,
}

pub enum IsLiked {
    Post,
    Comment,
    Photo,
    Video,
    Note,
    PhotoComment,
    VideoComment,
    TopicComment,
}

pub enum Filter {
    Likes,
    Copies,
}

pub struct GetListOptions {
    pub owner_id: i64,
    pub item_id: usize,
    pub page_url: String,
    pub filter: Filter,
    pub friends_only: bool,
    pub extended: bool,
    pub offset: usize,
    pub count: usize,
    pub skip_own: bool,
}

impl Default for GetListOptions {
    fn default() -> Self {
        Self {
            owner_id: 0,
            item_id: 0,
            page_url: "".to_string(),
            filter: Filter::Likes,
            friends_only: false,
            extended: false,
            offset: 0,
            count: 10,
            skip_own: false,
        }
    }
}

pub struct IsLikedOptions {
    pub user_id: usize,
}

impl Default for IsLikedOptions {
    fn default() -> Self {
        Self { user_id: 0 }
    }
}

pub struct AddDeleteOptions {
    pub from_group: bool,
    pub access_key: String,
}

impl Default for AddDeleteOptions {
    fn default() -> Self {
        Self {
            from_group: false,
            access_key: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsLikedResult {
    pub liked: i8,
    pub copied: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeleteResult {
    pub likes: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetListResultSimple {
    pub count: usize,
    pub items: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetListResultComplex {
    pub count: usize,
    pub items: Vec<Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: usize,
    #[serde(rename = "type")]
    pub profile_type: String,
    pub first_name: String,
    pub last_name: String,
    pub can_access_closed: bool,
    pub is_closed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetListResult {
    Simple(GetListResultSimple),
    Complex(GetListResultComplex),
}
