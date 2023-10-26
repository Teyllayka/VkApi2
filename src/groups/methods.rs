use crate::error::{VkApiError, VkError};
use crate::groups::types::{GetOnlineStatusResponse, JoinOptions, Link};
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/groups.";

pub async fn add_link(
    api: &VkApi,
    group_id: usize,
    link: String,
    text: String,
) -> Result<Link, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("link", link);
    params.insert_if_not_exists("text", text);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: Link = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn delete_link(api: &VkApi, group_id: usize, link_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("link_id", link_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn delete_address(
    api: &VkApi,
    group_id: usize,
    address_id: usize,
) -> Result<u8, VkApiError> {
    let api_key = if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("address_id", address_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteAddress", API),
        api_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn disable_online(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}disableOnline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn enable_online(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}enableOnline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn invite(api: &VkApi, group_id: usize, user_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("user_id", user_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}invite", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn leave(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}leave", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn remove_user(api: &VkApi, group_id: usize, user_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("user_id", user_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}removeUser", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn unban(api: &VkApi, group_id: usize, owner_id: i64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}unban", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn join(api: &VkApi, group_id: usize, options: JoinOptions) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    params.insert_if_not_exists(
        "not_sure",
        match options.not_sure {
            true => "1".to_string(),
            false => "0".to_string(),
        },
    );

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}join", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get_online_status(
    api: &VkApi,
    group_id: usize,
) -> Result<GetOnlineStatusResponse, VkApiError> {
    let api_key = if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getOnlineStatus", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetOnlineStatusResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn edit_link(
    api: &VkApi,
    group_id: usize,
    link_id: usize,
    text: String,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("link_id", link_id.to_string());
    params.insert_if_not_exists("text", text);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}editLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
