const API: &str = "https://api.vk.com/method/likes.";

use crate::likes::types::{
    AddDeleteOptions, AddDeleteResult, AddType, DeleteType, Filter, GetList, GetListOptions,
    GetListResult, IsLiked, IsLikedOptions, IsLikedResult,
};
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};
use serde_json::Value;

pub async fn add(
    api: &VkApi,
    post_type: AddType,
    owner_id: i64,
    item_id: usize,
    options: Option<AddDeleteOptions>,
) -> Result<AddDeleteResult, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("item_id", item_id.to_string());

    match post_type {
        AddType::Post => {
            params.insert_if_not_exists("type", "post");
        }
        AddType::Comment => {
            params.insert_if_not_exists("type", "comment");
        }
        AddType::Photo => {
            params.insert_if_not_exists("type", "photo");
        }
        AddType::Audio => {
            params.insert_if_not_exists("type", "audio");
        }
        AddType::Video => {
            params.insert_if_not_exists("type", "video");
        }
        AddType::Note => {
            params.insert_if_not_exists("type", "note");
        }
        AddType::Market => {
            params.insert_if_not_exists("type", "market");
        }
        AddType::PhotoComment => {
            params.insert_if_not_exists("type", "photo_comment");
        }
        AddType::VideoComment => {
            params.insert_if_not_exists("type", "video_comment");
        }
        AddType::TopicComment => {
            params.insert_if_not_exists("type", "topic_comment");
        }
        AddType::MarketComment => {
            params.insert_if_not_exists("type", "market_comment");
        }
    }

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
        params.insert_if_not_exists("from_group", options.from_group);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}add", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: AddDeleteResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn delete(
    api: &VkApi,
    post_type: DeleteType,
    owner_id: i64,
    item_id: usize,
    options: Option<AddDeleteOptions>,
) -> Result<AddDeleteResult, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("item_id", item_id.to_string());

    match post_type {
        DeleteType::Post => {
            params.insert_if_not_exists("type", "post");
        }
        DeleteType::Story => {
            params.insert_if_not_exists("type", "story");
        }
        DeleteType::Comment => {
            params.insert_if_not_exists("type", "comment");
        }
        DeleteType::Photo => {
            params.insert_if_not_exists("type", "photo");
        }
        DeleteType::Audio => {
            params.insert_if_not_exists("type", "audio");
        }
        DeleteType::Video => {
            params.insert_if_not_exists("type", "video");
        }
        DeleteType::Note => {
            params.insert_if_not_exists("type", "note");
        }
        DeleteType::Market => {
            params.insert_if_not_exists("type", "market");
        }
        DeleteType::PhotoComment => {
            params.insert_if_not_exists("type", "photo_comment");
        }
        DeleteType::VideoComment => {
            params.insert_if_not_exists("type", "video_comment");
        }
        DeleteType::TopicComment => {
            params.insert_if_not_exists("type", "topic_comment");
        }
        DeleteType::MarketComment => {
            params.insert_if_not_exists("type", "market_comment");
        }
        DeleteType::SitePage => {
            params.insert_if_not_exists("type", "sitepage");
        }
    }

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
        params.insert_if_not_exists("from_group", options.from_group);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}delete", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: AddDeleteResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn get_list(
    api: &VkApi,
    post_type: GetList,
    options: Option<GetListOptions>,
) -> Result<GetListResult, VkApiError> {
    let mut params = ParamGrid::new();

    match post_type {
        GetList::Post => {
            params.insert_if_not_exists("type", "post");
        }
        GetList::PostAds => {
            params.insert_if_not_exists("type", "post_ads");
        }
        GetList::Comment => {
            params.insert_if_not_exists("type", "comment");
        }
        GetList::Photo => {
            params.insert_if_not_exists("type", "photo");
        }
        GetList::Audio => {
            params.insert_if_not_exists("type", "audio");
        }
        GetList::Video => {
            params.insert_if_not_exists("type", "video");
        }
        GetList::Note => {
            params.insert_if_not_exists("type", "note");
        }
        GetList::Market => {
            params.insert_if_not_exists("type", "market");
        }
        GetList::PhotoComment => {
            params.insert_if_not_exists("type", "photo_comment");
        }
        GetList::VideoComment => {
            params.insert_if_not_exists("type", "video_comment");
        }
        GetList::TopicComment => {
            params.insert_if_not_exists("type", "topic_comment");
        }
        GetList::MarketComment => {
            params.insert_if_not_exists("type", "market_comment");
        }
        GetList::SitePage => {
            params.insert_if_not_exists("type", "sitepage");
        }
    }

    if let Some(options) = options {
        params.insert_if_not_exists("page_url", options.page_url);
        params.insert_if_not_exists("item_id", options.item_id.to_string());
        params.insert_if_not_exists("owner_id", options.owner_id.to_string());

        params.insert_if_not_exists(
            "filter",
            match options.filter {
                Filter::Likes => "likes",
                Filter::Copies => "copies",
            },
        );
        params.insert_if_not_exists(
            "friends_only",
            match options.friends_only {
                true => "1",
                false => "0",
            },
        );
        params.insert_if_not_exists(
            "extended",
            match options.extended {
                true => "1",
                false => "0",
            },
        );
        params.insert_if_not_exists("offset", options.offset.to_string());
        params.insert_if_not_exists("count", options.count.to_string());
        params.insert_if_not_exists("skip_own", options.skip_own);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getList", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetListResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn is_liked(
    api: &VkApi,
    post_type: IsLiked,
    owner_id: i64,
    item_id: usize,
    options: Option<IsLikedOptions>,
) -> Result<IsLikedResult, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("owner_id", owner_id.to_string());
    params.insert_if_not_exists("item_id", item_id.to_string());

    match post_type {
        IsLiked::Post => {
            params.insert_if_not_exists("type", "post");
        }
        IsLiked::Comment => {
            params.insert_if_not_exists("type", "comment");
        }
        IsLiked::Photo => {
            params.insert_if_not_exists("type", "photo");
        }
        IsLiked::Video => {
            params.insert_if_not_exists("type", "video");
        }
        IsLiked::Note => {
            params.insert_if_not_exists("type", "note");
        }
        IsLiked::PhotoComment => {
            params.insert_if_not_exists("type", "photo_comment");
        }
        IsLiked::VideoComment => {
            params.insert_if_not_exists("type", "video_comment");
        }
        IsLiked::TopicComment => {
            params.insert_if_not_exists("type", "topic_comment");
        }
    }

    if let Some(options) = options {
        params.insert_if_not_exists("user_id", options.user_id.to_string());
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}isLiked", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: IsLikedResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
