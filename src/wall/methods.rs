use crate::error::{VkApiError, VkError};
use crate::{send_request, ParamGrid, VkApi};

const API: &str = "https://api.vk.com/method/wall.";
pub async fn check_copyright_link(api: &VkApi, link: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("link", link);
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}checkCopyrightLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}

pub async fn close_comments(api: &VkApi, owner_id: i64, post_id: usize) -> Result<u8, VkApiError> {
    let api_key = if api.flow_key.is_empty() {
        &api.flow_key
    } else if api.group_key.is_empty() {
        &api.group_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("post_id", post_id.to_string());
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}closeComments", API),
        api_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}

pub async fn open_comments(api: &VkApi, owner_id: i64, post_id: usize) -> Result<u8, VkApiError> {
    let api_key = if api.flow_key.is_empty() {
        &api.flow_key
    } else if api.group_key.is_empty() {
        &api.group_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("post_id", post_id.to_string());
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}closeComments", API),
        api_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }
    Ok(1)
}

pub async fn restore(api: &VkApi, owner_id: i64, post_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("post_id", post_id.to_string());
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

pub async fn restore_comment(
    api: &VkApi,
    owner_id: i64,
    comment_id: usize,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("comment_id", comment_id.to_string());
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

pub async fn unpin(api: &VkApi, owner_id: i64, post_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("post_id", post_id.to_string());
    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}unpin", API),
        &api.flow_key,
        api.v,
    )
    .await?;
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn delete(api: &VkApi, owner_id: i64, post_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("post_id", post_id.to_string());
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

pub async fn delete_comment(
    api: &VkApi,
    owner_id: i64,
    comment_id: usize,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("comment_id", comment_id.to_string());
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
