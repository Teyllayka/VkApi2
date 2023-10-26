use crate::friends::types::AddOptions;
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/friends.";

pub async fn add(
    api: &VkApi,
    user_id: usize,
    options: Option<AddOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("user_id", user_id.to_string());

    if let Some(options) = options {
        params.insert_if_not_exists(
            "follow",
            match options.follow {
                true => "1".to_string(),
                false => "0".to_string(),
            },
        );
        params.insert_if_not_exists("text", options.text);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}add", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: u8 = serde_json::from_value(json["response"].clone())?;
        return Ok(data);
    }
}
