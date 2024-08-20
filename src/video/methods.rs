use crate::{error::{VkApiError, VkError}, insert_params, send_request, ParamGrid, VkApi};
const API: &str = "https://api.vk.com/method/video.";

pub async fn delete_comment(
    api: &VkApi,
    owner_id: i64,
    comment_id: usize,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    insert_params!(&mut params, owner_id, comment_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteComment", API),
        &api.flow_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}

pub async fn report_comment(
    api: &VkApi,
    owner_id: i64,
    comment_id: usize,
    reason: u8,
) -> Result<u8, VkApiError> {

    let mut params = ParamGrid::new();

    insert_params!(&mut params, owner_id, comment_id, reason);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}reportComment", API),
        &api.flow_key,
        api.v,
    ).await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }


    Ok(1)
}

pub async fn restore_comment(
    api: &VkApi,
    owner_id: i64,
    comment_id: usize,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    insert_params!(&mut params, owner_id, comment_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}restoreComment", API),
        &api.flow_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}


pub async fn restore(
    api: &VkApi,
    video_id: i64,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    insert_params!(&mut params, video_id, owner_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}restore", API),
        &api.flow_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}

