use crate::error::{VkApiError, VkError};
use crate::{ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/board.";


pub async fn unfix_topic(api: &VkApi, group_id:usize, topic_id:usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("group_id", group_id);
    params.insert_if_not_exists("topic_id", topic_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}unfixTopic", API),
        &api.flow_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}