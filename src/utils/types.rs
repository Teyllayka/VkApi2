use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetLinkStatsResponse {
    pub key: String,
    pub stats: Vec<Stats>,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub timestamp: usize,
    pub views: usize,
    pub cities: Option<Vec<City>>,
    pub countries: Option<Vec<Country>>,
    pub sex_age: Option<Vec<SexAge>>,
}

#[derive(Deserialize, Debug)]
pub struct City {
    pub city_id: i64,
    pub views: usize,
}

#[derive(Deserialize, Debug)]
pub struct Country {
    pub country_id: i64,
    pub views: usize,
}

#[derive(Deserialize, Debug)]
pub struct SexAge {
    pub age_range: String,
    pub male: usize,
    pub female: usize,
}

pub struct GetLinkStatsOptions {
    pub access_key: String,
    pub interval: Interval,
    pub intervals_count: usize,
    pub extended: bool,
}

pub enum Interval {
    Hour,
    Day,
    Week,
    Month,
    Forever,
}

impl Default for GetLinkStatsOptions {
    fn default() -> Self {
        Self {
            access_key: "".to_string(),
            interval: Interval::Hour,
            intervals_count: 1,
            extended: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckLinkResponse {
    pub status: String,
    pub link: String,
}

pub enum Private {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveScreenNameResponse {
    #[serde(rename = "type")]
    pub type_: ResolveScreenNameType,
    pub object_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResolveScreenNameType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "vk_app")]
    VkApp,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GetLastShortenedLinksResponse {
    pub count: usize,
    pub items: Vec<ShortenedLink>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ShortenedLink {
    pub timestamp: usize,
    pub url: String,
    pub short_url: String,
    pub key: String,
    pub views: usize,
    pub access_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetShortLinkResponse {
    pub short_url: String,
    pub access_key: Option<String>,
    pub key: String,
    pub url: String,
}

impl GetShortLinkResponse {
    pub fn get_statistics_url(&self) -> String {
        return format!(
            "https://vk.com/cc?act=stats&key={}{}",
            &self.short_url[14..],
            match &self.access_key {
                Some(access_key) => format!("&access_key={}", access_key),
                None => "".to_string(),
            }
        );
    }
}
