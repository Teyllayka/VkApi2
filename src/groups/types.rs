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

#[derive(Debug, Deserialize)]
pub struct City {
    pub id: usize,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct Country {
    pub id: usize,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct AddAddressResponse {
    pub id: usize,
    pub address: String,
    pub city_id: usize,
    pub country_id: usize,
    pub city: City,
    pub country: Country,
    pub latitude: f32,
    pub longitude: f32,
    pub title: String,
    pub work_info_status: String,
}

pub struct AddAddressOptions {
    pub additional_address: String,
    pub work_info_status: String,
    //pub timetable: String,
    pub phone: String,
    pub is_main_address: bool,
}

impl Default for AddAddressOptions {
    fn default() -> Self {
        Self {
            additional_address: "".to_string(),
            work_info_status: "".to_string(),
            //timetable: "".to_string(),
            phone: "".to_string(),
            is_main_address: false,
        }
    }
}

pub struct BanOptions {
    pub comment: String,
    pub end_date: usize,
    pub reason: u8,
    pub comment_visible: bool,
}

impl Default for BanOptions {
    fn default() -> Self {
        Self {
            comment: "".to_string(),
            end_date: 0,
            reason: 0,
            comment_visible: false,
        }
    }
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
