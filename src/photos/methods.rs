use crate::photos::types::ReorderAlbumsPhotosOptions;
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/photos.";

pub async fn confirm_tag(
    api: &VkApi,
    photo_id: String,
    tag_id: usize,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("photo_id", photo_id);
    params.insert_if_not_exists("tag_id", tag_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}confirmTag", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn move_(
    api: &VkApi,
    target_album_id: usize,
    photo_id: String,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("photo_id", photo_id);
    params.insert_if_not_exists("target_album_id", target_album_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}move", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn remove_tag(
    api: &VkApi,
    photo_id: String,
    tag_id: usize,
    owner_id: i64,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("photo_id", photo_id);
    params.insert_if_not_exists("tag_id", tag_id.to_string());

    params.insert_if_not_exists("owner_id", owner_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}removeTag", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn reorder_albums(
    api: &VkApi,
    album_id: usize,
    owner_id: i64,
    options: Option<ReorderAlbumsPhotosOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("album_id", album_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    match options {
        Some(options) => {
            params.insert_if_not_exists("before", options.before.to_string());
            params.insert_if_not_exists("after", options.after.to_string());
        }
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}reorderAlbums", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn reorder_photos(
    api: &VkApi,
    photo_id: usize,
    owner_id: i64,
    options: Option<ReorderAlbumsPhotosOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("photo_id", photo_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

    match options {
        Some(options) => {
            params.insert_if_not_exists("before", options.before.to_string());
            params.insert_if_not_exists("after", options.after.to_string());
        }
        None => {}
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}reorderPhotos", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
