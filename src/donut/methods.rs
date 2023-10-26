use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};
use serde_json::Value;
const API: &str = "https://api.vk.com/method/donut.";

pub async fn is_don(api: &VkApi, owner_id: i64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}isDon", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let is: u8 = serde_json::from_value(json["response"].clone())?;
    Ok(is)
}
