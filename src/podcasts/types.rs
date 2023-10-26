use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PodcastResult {
    pub results_total: usize,
    pub podcasts: Vec<Podcast>,
}
#[derive(Debug, Deserialize)]
pub struct Podcast {
    pub url: String,
    pub owner_url: String,
    pub title: String,
    pub owner_name: String,
    pub cover: Cover,
}
#[derive(Debug, Deserialize)]
pub struct Cover {
    pub sizes: Vec<Sizes>,
}
#[derive(Debug, Deserialize)]
pub struct Sizes {
    pub height: usize,
    pub width: usize,
    #[serde(alias = "type")]
    pub type_: String,
    pub url: String,
}
