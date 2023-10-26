use crate::error::{VkApiError, VkError};
use crate::podcasts::types::PodcastResult;
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/podcasts.";

pub async fn search_podcast(
    api: &VkApi,
    search_string: String,
    offset: i32,
    count: i32,
) -> Result<PodcastResult, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("search_string", search_string);
    params.insert_if_not_exists("offset", offset);
    params.insert_if_not_exists("count", count);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}searchPodcast", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: PodcastResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
