use serde_json::Value;

use crate::status::types::StatusGetResponse;
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/status.";

pub async fn set(api: &VkApi, text: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("text", text.to_string());
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}set", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get(api: &VkApi, user_id: usize) -> Result<StatusGetResponse, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("user_id", user_id.to_string());
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}get", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: StatusGetResponse = serde_json::from_value(json["response"].clone())?;
        return Ok(data);
    }
}
