use crate::error::{VkApiError, VkError};
use crate::translations::types::TranslateResult;
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/translations.";

pub async fn translate(
    api: &VkApi,
    texts: Vec<String>,
    translation_language: String,
) -> Result<TranslateResult, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("texts", texts.join(","));
    params.insert_if_not_exists("translation_language", translation_language);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}translate", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: TranslateResult = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}
