use crate::error::{VkApiError, VkError};
use crate::gifts::types::GiftsResult;
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/gifts.";

pub async fn get(
    api: &VkApi,
    user_id: usize,
    count: usize,
    offset: usize,
) -> Result<GiftsResult, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("user_id", user_id.to_string());
    params.insert_if_not_exists("count", count.to_string());
    params.insert_if_not_exists("offset", offset.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}get", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GiftsResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
