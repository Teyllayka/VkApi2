use crate::error::{VkApiError, VkError};
use crate::{ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/market.";


pub async fn delete(api: &VkApi, owner_id:i64, item_id:usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);
    params.insert_if_not_exists("item_id", item_id);

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

pub async fn delete_property(api: &VkApi, group_id:usize, property_id:usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("group_id", group_id);
    params.insert_if_not_exists("property_id", property_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteProperty", API),
        &api.flow_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn ungroup_items(api: &VkApi, group_id:i64, item_group_id:usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("group_id", group_id);
    params.insert_if_not_exists("item_group_id", item_group_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}ungroupItems", API),
        &api.flow_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}