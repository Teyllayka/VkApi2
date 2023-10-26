use serde::{Deserialize, Serialize};

// fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let opt = Option::deserialize(deserializer)?;
//     match opt {
//         Some(val) => match val {
//             0 => Ok(Some(false)),
//             1 => Ok(Some(true)),
//             _ => Ok(None),
//         },
//         None => Ok(None),
//     }
// }

pub struct UsersReportOptions {
    pub comment: String,
}

impl Default for UsersReportOptions {
    fn default() -> Self {
        Self {
            comment: "".to_string(),
        }
    }
}

pub enum User {
    Id(usize),
    ScreenName(String),
}

pub enum Fields {
    Activities,
    About,
    Blacklisted,
    BlacklistedByMe,
    Books,
    Bdate,
    CanBeInvitedGroup,
    CanPost,
    CanSeeAllPosts,
    CanSeeAudio,
    CanSendFriendRequest,
    CanWritePrivateMessage,
    Career,
    CommonCount,
    Connections,
    Contacts,
    City,
    Country,
    CropPhoto,
    Domain,
    Education,
    Exports,
    FollowersCount,
    FriendStatus,
    HasPhoto,
    HasMobile,
    HomeTown,
    Photo100,
    Photo200,
    Photo200Orig,
    Photo4000rig,
    Photo50,
    Sex,
    Site,
    Schools,
    ScreenName,
    Status,
    Verified,
    Games,
    Interests,
    IsFavorite,
    IsFriend,
    IsHiddenFromFeed,
    LastSeen,
    MaidenName,
    Military,
    Movies,
    Music,
    Nickname,
    Occupation,
    Online,
    Personal,
    PhotoId,
    PhotoMax,
    PhotoMaxOrig,
    Quotes,
    Relation,
    Relatives,
    Timezone,
    Tv,
    Universities,
}

pub enum NameCase {
    Nom,
    Gen,
    Dat,
    Acc,
    Ins,
    Abl,
}

pub enum ReportType {
    Porn,
    Spam,
    Insult,
    Advertisement,
}
#[derive(Debug, Deserialize, Serialize)]

pub struct UserResponse {
    pub response: Vec<UserProfile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowersResponse {
    pub items: Vec<UserProfile>,
    pub count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfile {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub can_access_closed: bool,
    pub is_closed: bool,
    pub about: Option<String>,
    pub activities: Option<String>,
    pub bdate: Option<String>,
    pub blacklisted: Option<usize>,
    pub blacklisted_by_me: Option<usize>,
    pub photo_50: Option<String>,
    pub books: Option<String>,
    pub can_be_invited_group: Option<bool>,
    pub can_post: Option<usize>,
    pub can_see_all_posts: Option<usize>,
    pub can_see_audio: Option<usize>,
    pub can_send_friend_request: Option<usize>,
    pub can_write_private_message: Option<usize>,
    pub career: Option<Vec<Career>>,
    pub common_count: Option<usize>,
    pub connections: Option<String>,
    pub mobile_phone: Option<String>,
    pub home_phone: Option<String>,
    pub city: Option<City>,
    pub country: Option<Country>,
    pub crop_photo: Option<Crop>,
    pub domain: Option<String>,
    pub university: Option<usize>,
    pub university_name: Option<String>,
    pub faculty: Option<usize>,
    pub faculty_name: Option<String>,
    pub graduation: Option<usize>,
    pub exports: Option<String>,
    pub followers_count: Option<usize>,
    pub friend_status: Option<usize>,
    pub has_photo: Option<usize>,
    pub has_mobile: Option<usize>,
    pub home_town: Option<String>,
    pub photo_100: Option<String>,
    pub photo_200: Option<String>,
    pub photo_200_orig: Option<String>,
    pub photo_400_orig: Option<String>,
    pub sex: Option<usize>,
    pub site: Option<String>,
    pub schools: Option<Vec<School>>,
    pub screen_name: Option<String>,
    pub status: Option<String>,
    pub verified: Option<usize>,
    pub games: Option<String>,
    pub interests: Option<String>,
    pub is_favorite: Option<usize>,
    pub is_friend: Option<usize>,
    pub is_hidden_from_feed: Option<usize>,
    pub last_seen: Option<LastSeen>,
    pub maiden_name: Option<String>,
    pub military: Option<Vec<Military>>,
    pub movies: Option<String>,
    pub music: Option<String>,
    pub nickname: Option<String>,
    pub occupation: Option<Occupation>,
    pub online: Option<usize>,
    pub personal: Option<Personal>,
    pub photo_id: Option<String>,
    pub photo_max: Option<String>,
    pub photo_max_orig: Option<String>,
    pub quotes: Option<String>,
    pub relation: Option<usize>,
    pub relation_partner: Option<RelationPartner>,
    pub relatives: Option<Vec<Relatives>>,
    pub timezone: Option<usize>,
    pub tv: Option<String>,
    pub universities: Option<Vec<Universities>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Universities {
    pub id: usize,
    pub country: usize,
    pub city: usize,
    pub name: Option<String>,
    pub faculty: Option<usize>,
    pub faculty_name: Option<String>,
    pub chair: Option<usize>,
    pub chair_name: Option<String>,
    pub graduation: Option<usize>,
    pub education_form: Option<String>,
    pub education_status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Relatives {
    pub name: String,
    pub id: usize,
    #[serde(alias = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelationPartner {
    pub first_name: String,
    pub id: usize,
    pub last_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Personal {
    pub political: Option<usize>,
    pub religion: Option<String>,
    pub inspired_by: Option<String>,
    pub langs: Option<Vec<String>>,
    pub life_main: Option<usize>,
    pub smoking: Option<usize>,
    pub alcohol: Option<usize>,
    pub people_main: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Occupation {
    #[serde(alias = "type")]
    pub type_: Option<String>,
    pub id: Option<usize>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Military {
    pub unit: Option<String>,
    pub unit_id: Option<usize>,
    pub country_id: Option<usize>,
    pub from: Option<usize>,
    pub until: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastSeen {
    pub time: usize,
    pub platform: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct School {
    pub id: String,
    pub country: usize,
    pub city: usize,
    pub name: String,
    pub year_from: Option<usize>,
    pub year_to: Option<usize>,
    pub year_graduated: Option<usize>,
    pub class: Option<String>,
    pub speciality: Option<String>,
    #[serde(alias = "type")]
    pub type_: Option<usize>,
    pub type_str: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Crop {
    pub crop: Coordinates,
    pub photo: Photo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Photo {
    pub album_id: i64,
    pub date: usize,
    pub has_tags: bool,
    pub id: usize,
    pub owner_id: usize,
    pub post_id: usize,
    pub sizes: Vec<Sizes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sizes {
    pub height: usize,
    #[serde(alias = "type")]
    pub type_: String,
    pub url: String,
    pub width: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: f32,
    pub x2: f32,
    pub y: f32,
    pub y2: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    pub id: usize,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub id: usize,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Career {
    pub group_id: Option<usize>,
    pub company: Option<String>,
    pub country_id: usize,
    pub city_id: Option<usize>,
    pub city_name: Option<String>,
    pub from: Option<usize>,
    pub until: Option<usize>,
    pub position: Option<String>,
}

pub struct UserGetOptions {
    pub user_ids: Vec<User>,
    pub fields: Vec<Fields>,
    pub name_case: NameCase,
}

pub struct UserGetFollowersOptions {
    pub fields: Vec<Fields>,
    pub name_case: NameCase,
}

impl Default for UserGetFollowersOptions {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            name_case: NameCase::Nom,
        }
    }
}

impl Default for UserGetOptions {
    fn default() -> Self {
        Self {
            user_ids: Vec::new(),
            fields: Vec::new(),
            name_case: NameCase::Nom,
        }
    }
}
