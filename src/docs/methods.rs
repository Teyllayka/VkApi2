use crate::docs::types::AddDocsOptions;
use crate::error::{VkApiError, VkError};
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/docs.";

pub async fn add(
    api: &VkApi,
    owner_id: i64,
    doc_id: usize,
    options: Option<AddDocsOptions>,
) -> Result<usize, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);

    params.insert_if_not_exists("doc_id", doc_id);

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
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
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let id: usize = serde_json::from_value(json["response"].clone())?;
    Ok(id)
}

pub async fn delete(api: &VkApi, owner_id: i64, doc_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);
    params.insert_if_not_exists("doc_id", doc_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}delete", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
