use crate::fave::types::{AddPage, AddPostOptions, AddProductVideoOptions};
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/fave.";

pub async fn add_article(api: &VkApi, url: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("url", url);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addArticle", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn add_link(api: &VkApi, link: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("link", link);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn add_page(api: &VkApi, add_page: AddPage) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    match add_page {
        AddPage::User(id) => params.insert_if_not_exists("user_id", id.to_string()),
        AddPage::Group(id) => params.insert_if_not_exists("group_id", id.to_string()),
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addPage", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn add_product(
    api: &VkApi,
    owner_id: i64,
    id: usize,
    options: Option<AddProductVideoOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
    }

    params.insert_if_not_exists("owner_id", owner_id.to_string());

    params.insert_if_not_exists("id", id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addProduct", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn add_post(
    api: &VkApi,
    owner_id: i64,
    id: usize,
    options: Option<AddPostOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
        params.insert_if_not_exists("ref", options.option_ref);
        params.insert_if_not_exists("track_code", options.track_code);
        params.insert_if_not_exists("source", options.source);
    }

    params.insert_if_not_exists("owner_id", owner_id.to_string());

    params.insert_if_not_exists("id", id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addPost", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn add_video(
    api: &VkApi,
    owner_id: i64,
    id: usize,
    options: Option<AddProductVideoOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    if let Some(options) = options {
        params.insert_if_not_exists("access_key", options.access_key);
    }

    params.insert_if_not_exists("owner_id", owner_id.to_string());

    params.insert_if_not_exists("id", id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addVideo", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn edit_tag(api: &VkApi, id: usize, name: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("name", name);

    params.insert_if_not_exists("id", id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}editTag", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
