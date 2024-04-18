use crate::error::{VkApiError, VkError};
use crate::groups::types::{AddAddressOptions, AddAddressResponse, BanOptions, GetOnlineStatusResponse, JoinOptions, Link};
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/groups.";






pub async fn add_address(api: &VkApi, group_id: usize, title: String, address: String, country_id: i32, city_id: i32, latitude: i8, longitude: i8, options: Option<AddAddressOptions>) -> Result<AddAddressResponse, VkApiError> {
    let mut params = ParamGrid::new();

    let api_key = if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("address", address);
    params.insert_if_not_exists("title", title);
    params.insert_if_not_exists("country_id", country_id);
    params.insert_if_not_exists("city_id", city_id);
    params.insert_if_not_exists("latitude", latitude);
    params.insert_if_not_exists("longitude", longitude);

    if let Some(options) = options {
        params.insert_if_not_exists("additional_address", options.additional_address);
        params.insert_if_not_exists("work_info_status", options.work_info_status);
        //params.insert_if_not_exists("timetable", options.timetable);
        params.insert_if_not_exists("phone", options.phone);
        params.insert_if_not_exists("is_main_address", match options.is_main_address {
            true => "1".to_string(),
            false => "0".to_string(),
        });
    }






    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addAddress", API),
        &api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: AddAddressResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}



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

pub async fn ban(api: &VkApi, group_id: usize, owner_id: i64, options: BanOptions) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id);
    params.insert_if_not_exists("owner_id", owner_id);

    if let Some(options) = options {
        params.insert_if_not_exists("comment", options.comment);
        params.insert_if_not_exists("end_date", options.end_date);
        params.insert_if_not_exists("comment_visible", match options.comment_visible {
            true => "1".to_string(),
            false => "0".to_string(),
        });
        params.insert_if_not_exists("reason", options.reason);

    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}ban", API),
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
