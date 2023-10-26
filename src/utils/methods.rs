const API: &str = "https://api.vk.com/method/utils.";

use crate::utils::types::{
    CheckLinkResponse, GetLastShortenedLinksResponse, GetLinkStatsOptions, GetLinkStatsResponse,
    GetShortLinkResponse, Interval, Private, ResolveScreenNameResponse,
};
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};
use serde_json::Value;

pub async fn check_link(api: &VkApi, url: String) -> Result<CheckLinkResponse, VkApiError> {
    let api_key = if !api.service_key.is_empty() {
        &api.service_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("url", url);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}checkLink", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: CheckLinkResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn delete_from_last_shortened(api: &VkApi, key: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("key", key);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteFromLastShortened", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get_last_shortened_links(
    api: &VkApi,
    count: usize,
    offset: usize,
) -> Result<GetLastShortenedLinksResponse, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("count", count.to_string());
    params.insert_if_not_exists("offset", offset.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getLastShortenedLinks", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetLastShortenedLinksResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn get_link_stats(
    api: &VkApi,
    key: String,
    options: Option<GetLinkStatsOptions>,
) -> Result<GetLinkStatsResponse, VkApiError> {
    let api_key = if !api.service_key.is_empty() {
        &api.service_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("key", key);

    match options {
        Some(options) => {
            params.insert_if_not_exists(
                "interval",
                match options.interval {
                    Interval::Hour => "hour",
                    Interval::Day => "day",
                    Interval::Week => "week",
                    Interval::Month => "month",
                    Interval::Forever => "forever",
                },
            );

            params.insert_if_not_exists("intervals_count", options.intervals_count);

            params.insert_if_not_exists(
                "extended",
                match options.extended {
                    true => "1",
                    false => "0",
                },
            );

            params.insert_if_not_exists("access_key", options.access_key);
        }
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getLinkStats", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetLinkStatsResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn get_server_time(api: &VkApi) -> Result<usize, VkApiError> {
    let api_key = if !api.service_key.is_empty() {
        &api.service_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let response_text = send_request(
        &api.client,
        None,
        &format!("{}getServerTime", API),
        api_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let time: usize = serde_json::from_value(json["response"].clone())?;
    Ok(time)
}

pub async fn get_short_link(
    api: &VkApi,
    url: String,
    private: Private,
) -> Result<GetShortLinkResponse, VkApiError> {
    let api_key = if !api.flow_key.is_empty() {
        &api.flow_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.service_key.is_empty() {
        &api.service_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("url", url);

    match private {
        Private::Yes => params.insert_if_not_exists("private", "1"),
        Private::No => params.insert_if_not_exists("private", "0"),
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getShortLink", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetShortLinkResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn resolve_screen_name(
    api: &VkApi,
    screen_name: String,
) -> Result<ResolveScreenNameResponse, VkApiError> {
    let api_key = if !api.flow_key.is_empty() {
        &api.flow_key
    } else if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.service_key.is_empty() {
        &api.service_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();
    params.insert_if_not_exists("screen_name", screen_name);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}resolveScreenName", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: ResolveScreenNameResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
