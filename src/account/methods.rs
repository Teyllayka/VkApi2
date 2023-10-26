use crate::account::types::{
    GetInfoOptions, ResponseBanned, ResponseInfo, SetOnlineOptions, UnregisterDeviceOptions,
};
use crate::error::{VkApiError, VkError};
use crate::{send_request, ParamGrid, VkApi};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/account.";

pub async fn ban(api: &VkApi, owner_id: i64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}ban", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn unban(api: &VkApi, owner_id: i64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();
    params.insert_if_not_exists("owner_id", owner_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}unban", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn set_offline(api: &VkApi) -> Result<u8, VkApiError> {
    let response_text = send_request(
        &api.client,
        None,
        &format!("{}setOffline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn set_online(api: &VkApi, options: Option<SetOnlineOptions>) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    match options {
        Some(options) => {
            params.insert_if_not_exists(
                "voip",
                match options.voip {
                    true => "1".to_string(),
                    false => "0".to_string(),
                },
            );
        }
        None => (),
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}setOnline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get_info(
    api: &VkApi,
    options: Option<GetInfoOptions>,
) -> Result<ResponseInfo, VkApiError> {
    let mut params = ParamGrid::new();
    let mut fields = String::new();

    match options {
        Some(options) => {
            if options.country {
                fields.push_str("country,");
            };
            if options.https_required {
                fields.push_str("https_required,");
            };
            if options.intro {
                fields.push_str("intro,");
            };
            if options.lang {
                fields.push_str("lang,");
            };
            if options.no_wall_replies {
                fields.push_str("no_wall_replies,");
            };
            if options.own_posts_default {
                fields.push_str("own_posts_default,");
            };
        }
        None => (),
    };

    params.insert_if_not_exists("fields", fields);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getInfo", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: ResponseInfo = serde_json::from_value(json["response"].clone())?;
        return Ok(data);
    }
}

pub async fn change_password() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn get_active_offers() -> Result<(), VkApiError> {
    // let mut params = ParamGrid::new();

    // params.insert_if_not_exists("offset", offset);
    // params.insert_if_not_exists("count", count);

    // let response_text = send_request(
    //     &api.client,
    //     Some(params),
    //     &format!("{}getInfo", API),
    //     &api.flow_key,
    //     api.v,
    // )
    // .await?;

    // if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
    //     return Err(VkApiError::VkError(error));
    // } else {
    //     let json: Value = serde_json::from_str(&response_text)?;
    //     let data: ResponseInfo = serde_json::from_value(json["response"].clone())?;
    //     return Ok(data);
    // }

    // Ok(())
    todo!()
}

pub async fn get_app_permissions() -> Result<(), VkApiError> {
    todo!()
}

pub async fn get_banned(
    api: &VkApi,
    offset: usize,
    count: usize,
) -> Result<ResponseBanned, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("offset", offset);
    params.insert_if_not_exists("count", count);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getBanned", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: ResponseBanned = serde_json::from_value(json["response"].clone())?;
        return Ok(data);
    }
}

pub async fn get_counters() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn get_profile_info() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn get_push_settings() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn lookup_contacts() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn register_device() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn save_profile_info() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn set_info() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn set_name_in_menu(api: &VkApi, user_id: usize, name: String) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("user_id", user_id);
    params.insert_if_not_exists("name", name);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}setNameInMenu", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn set_push_settings() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn set_silence_mode() -> Result<(), VkApiError> {
    // let mut params = match params {
    //     Some(params) => params,
    //     None => ParamGrid::new(),
    // };

    // params.insert_if_not_exists("v", api.v);

    // let response = api
    //     .client
    //     .post(format!("{}ban", API))
    //     .header("Authorization", format!("Bearer {}", api.flow_key))
    //     .form(&params.data)
    //     .send()
    //     .await?;

    // if let Ok(error) = response.json::<VkError>().await {
    //     return Err(VkApiError::VkError(error));
    // };

    // Ok(())
    todo!()
}

pub async fn unregister_device(
    api: &VkApi,
    device_id: i64,
    options: Option<UnregisterDeviceOptions>,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("device_id", device_id);

    match options {
        Some(options) => {
            params.insert_if_not_exists(
                "sandbox",
                match options.sandbox {
                    true => "1".to_string(),
                    false => "0".to_string(),
                },
            );
        }
        None => (),
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}unregisterDevice", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
