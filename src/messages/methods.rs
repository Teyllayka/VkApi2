use crate::messages::types::AddChatUserOptions;
use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/messages.";

pub async fn add_chat_user(
    api: &VkApi,
    chat_id: usize,
    user_id: usize,
    options: Option<AddChatUserOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("chat_id", chat_id);

    params.insert_if_not_exists("user_id", user_id);

    if let Some(options) = options {
        params.insert_if_not_exists("visible_messages_count", options.visible_messages_count);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addChatUser", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
//
//
// pub async fn restore(api: &VkApi, message_id: usize, group_id: usize) -> Result<u8, VkApiError> {
//     let mut params = ParamGrid::new();
//     params.insert_if_not_exists("message_id", message_id);
//     params.insert_if_not_exists("group_id", group_id);
//
//
//     let response_text = send_request(
//         &api.client,
//         Some(params),
//         &format!("{}restore", API),
//         &api.flow_key,
//         api.v,
//     )
//     .await?;
//
//     if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
//         return Err(VkApiError::VkError(error));
//     }
//
//     Ok(1)
// }
//


