pub enum AddPage {
    User(usize),
    Group(usize),
}

pub struct AddProductVideoOptions {
    pub access_key: String,
}

impl Default for AddProductVideoOptions {
    fn default() -> Self {
        Self {
            access_key: "".to_string(),
        }
    }
}

pub struct AddPostOptions {
    pub access_key: String,
    pub option_ref: String,
    pub track_code: String,
    pub source: String,
}

impl Default for AddPostOptions {
    fn default() -> Self {
        Self {
            access_key: "".to_string(),
            option_ref: "".to_string(),
            track_code: "".to_string(),
            source: "".to_string(),
        }
    }
}
