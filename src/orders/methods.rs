use crate::error::{VkApiError, VkError};
use crate::{ParamGrid, send_request, VkApi};

const API: &str = "https://api.vk.com/method/orders.";



pub async fn update_subscription(api: &VkApi, user_id:i64, subscription_id:usize, price:f64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("user_id", user_id);
    params.insert_if_not_exists("subscription_id", subscription_id);
    params.insert_if_not_exists("price", price);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}updateSubscription", API),
        &api.service_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
