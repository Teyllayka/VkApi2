use serde_json::Value;
use crate::error::{VkApiError, VkError};
use crate::{ParamGrid, send_request, VkApi};
use crate::streaming::types::MonthlyTier;

const API: &str = "https://api.vk.com/method/streaming.";



pub async fn get_stem(api: &VkApi, word: String) -> Result<String, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("word", word);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getStem", API),
        &api.service_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let word: String = serde_json::from_value(json["response"]["stem"].clone())?;
    Ok(word)
}


pub async fn set_settings(api: &VkApi, monthly_tier: MonthlyTier) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    match monthly_tier {
        MonthlyTier::Tier1 => params.insert_if_not_exists("monthly_tier", "tier_1"),
        MonthlyTier::Tier2 => params.insert_if_not_exists("monthly_tier", "tier_2"),
        MonthlyTier::Tier3 => params.insert_if_not_exists("monthly_tier", "tier_3"),
        MonthlyTier::Tier4 => params.insert_if_not_exists("monthly_tier", "tier_4"),
        MonthlyTier::Tier5 => params.insert_if_not_exists("monthly_tier", "tier_5"),
        MonthlyTier::Tier6 => params.insert_if_not_exists("monthly_tier", "tier_6"),
        MonthlyTier::Unlimited => params.insert_if_not_exists("monthly_tier", "unlimited"),

    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}setSettings", API),
        &api.service_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get_settings(api: &VkApi) -> Result<MonthlyTier, VkApiError> {
    let response_text = send_request(
        &api.client,
        None,
        &format!("{}getSettings", API),
        &api.service_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let tier: MonthlyTier = serde_json::from_value(json["response"]["monthly_limit"].clone())?;

    // match tier.as_str() {
    //     "tier_1" => Ok(MonthlyTier::Tier1),
    //     "tier_2" => Ok(MonthlyTier::Tier2),
    //     "tier_3" => Ok(MonthlyTier::Tier3),
    //     "tier_4" => Ok(MonthlyTier::Tier4),
    //     "tier_5" => Ok(MonthlyTier::Tier5),
    //     "tier_6" => Ok(MonthlyTier::Tier6),
    //     "unlimited" => Ok(MonthlyTier::Unlimited),
    //     _ => Err(VkApiError::VkError(VkError::UnknownError)),
    // }

    Ok(tier)
}