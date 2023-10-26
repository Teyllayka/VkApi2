use crate::error::{VkApiError, VkError};
use crate::{ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/utils.";


pub async fn hide_reply(api: &VkApi, owner_id: i64, story_id: usize) -> Result<u8, VkApiError> {
    let api_key = if !api.flow_key.is_empty() {
        &api.flow_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);
    params.insert_if_not_exists("story_id", story_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}banOwner", API),
        api_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}