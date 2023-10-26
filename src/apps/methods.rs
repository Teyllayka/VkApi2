use crate::error::{VkApiError, VkError};
use crate::{ send_request, VkApi};

const API: &str = "https://api.vk.com/method/apps.";


pub async fn delete_app_requests(api: &VkApi) -> Result<u8, VkApiError> {
    let response_text = send_request(
        &api.client,
        None,
        &format!("{}deleteAppRequests", API),
        &api.flow_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
