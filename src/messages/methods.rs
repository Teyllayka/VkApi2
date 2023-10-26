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

// pub async fn restore(api: &VkApi, params: Option<ParamGrid>) -> Result<u8, VkApiError> {
//     let mut params = match params {
//         Some(params) => params,
//         None => ParamGrid::new(),
//     };

//     params.insert_if_not_exists("v", api.v);

//     let key = match api.flow_key {
//         "".to_string() => api.group_key,
//         _ => api.flow_key
//     };

//     let response = api
//         .client
//         .post(format!("{}restore", API))
//         .header("Authorization", format!("Bearer {}", api.flow_key))
//         .form(&params.data)
//         .send()
//         .await?;

//     if let Ok(error) = response.json::<VkError>().await {
//         return Err(VkApiError::VkError(error));
//     };

//     Ok(1)
// }

// pub async fn remove_chat_user(api: &VkApi, params: Option<ParamGrid>) -> Result<u8, VkApiError> {
//     let mut params = match params {
//         Some(params) => params,
//         None => ParamGrid::new(),
//     };

//     params.insert_if_not_exists("v", api.v);

//     let key = match api.flow_key {
//         "".to_string() => api.group_key,
//         _ => api.flow_key
//     };

//     let response = api
//         .client
//         .post(format!("{}removeChatUser", API))
//         .header("Authorization", format!("Bearer {}", key))
//         .form(&params.data)
//         .send()
//         .await?;

//     if let Ok(error) = response.json::<VkError>().await {
//         return Err(VkApiError::VkError(error));
//     };

//     Ok(1)
// }
