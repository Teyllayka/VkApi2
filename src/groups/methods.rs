use crate::error::{VkApiError, VkError};
use crate::groups::types::{AddAddressOptions, AddAddressResponse, BanOptions, GetOnlineStatusResponse, JoinOptions, Link};
use crate::{send_request, ParamGrid, VkApi, insert_params, Api};
use serde_json::Value;

const API: &str = "https://api.vk.com/method/groups.";

pub async fn add_address(api: &VkApi, api_key: Api, group_id: usize, title: String, address: String, country_id: i32, city_id: i32, latitude: i8, longitude: i8, options: Option<AddAddressOptions>) -> Result<AddAddressResponse, VkApiError> {
    let mut params = ParamGrid::new();

    let api_key = match api_key {
      Api::Flow => {
          if !api.flow_key.is_empty() {
              &api.flow_key
          } else {
              return Err(VkApiError::InternalError("You did not provide flow key".to_string()));
          }
      },
        Api::Group => {
            if !api.group_key.is_empty() {
                &api.group_key
            } else {
                return Err(VkApiError::InternalError("You did not provide group key".to_string()));
            }
        }
        Api::Service => {
            return Err(VkApiError::InternalError("Service key is not supported for this method.".to_string()));
        }
    };


    insert_params!(&mut params, group_id, address, title, country_id, city_id, latitude, longitude);

    println!("{:?}", params);

    if let Some(options) = options {
        insert_params!(&mut params, options.additional_address, options.work_info_status, options.phone, match options.is_main_address {
            true => "1".to_string(),
            false => "0".to_string(),
        });
        //params.insert_if_not_exists("timetable", options.timetable);
    }

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addAddress", API),
        &api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: AddAddressResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}



pub async fn add_callback_server(api: &VkApi, api_key: Api, group_id: usize, url: String, title: String, secret_key: Option<String>) -> Result<i64, VkApiError> {
    let mut params = ParamGrid::new();

    let api_key = match api_key {
        Api::Flow => {
            if !api.flow_key.is_empty() {
                &api.flow_key
            } else {
                return Err(VkApiError::InternalError("You did not provide flow key".to_string()));
            }
        },
        Api::Group => {
            if !api.group_key.is_empty() {
                &api.group_key
            } else {
                return Err(VkApiError::InternalError("You did not provide group key".to_string()));
            }
        }
        Api::Service => {
            return Err(VkApiError::InternalError("Service key is not supported for this method.".to_string()));
        }
    };

    insert_params!(&mut params, group_id, url, title);

    if let Some(secret_key) = secret_key {
        params.insert_if_not_exists("secret_key", secret_key);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addCallbackServer", API),
        &api_key,
        api.v,
    )
    .await
    .unwrap();

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    let json: Value = serde_json::from_str(&response_text)?;

    let id: i64 = serde_json::from_value(json["response"].clone())?;
    Ok(id)

}


pub async fn add_link(
    api: &VkApi,
    group_id: usize,
    link: String,
    text: Option<String>,
) -> Result<Link, VkApiError> {
    let mut params = ParamGrid::new();

    insert_params!(&mut params, group_id, link);

    match text {
        Some(text) => {
            params.insert_if_not_exists("text", text);
        },
        None => {}
    }


    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}addLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: Link = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}


pub async fn approve_request(api: &VkApi, group_id: usize, user_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    insert_params!(&mut params, group_id, user_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}approveRequest", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        Ok(1)
    };
}

pub async fn ban(api: &VkApi, group_id: usize, owner_id: i64, options: Option<BanOptions>) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id);
    params.insert_if_not_exists("owner_id", owner_id);

    insert_params!(&mut params, group_id, owner_id);

    if let Some(options) = options {
        insert_params!(params, options.comment, options.end_date, match options.comment_visible {
            true => "1".to_string(),
            false => "0".to_string(),
        }, options.reason);
    }

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

pub async fn create() -> Result<i64, VkApiError> {
    // let mut params = ParamGrid::new();
    //
    // insert_params!(&mut params, title, description);
    //
    // if let Some(type_) = type_ {
    //     params.insert_if_not_exists("type", type_);
    // }
    //
    // let response_text = send_request(
    //     &api.client,
    //     Some(params),
    //     &format!("{}create", API),
    //     &api.flow_key,
    //     api.v,
    // )
    // .await?;
    //
    // return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
    //     Err(VkApiError::VkError(error))
    // } else {
    //     let json: Value = serde_json::from_str(&response_text)?;
    //     let id: i64 = serde_json::from_value(json["response"].clone())?;
    //     Ok(id)
    // };
    unimplemented!("This method is not implemented yet.");

}

pub async fn delete_address(
    api: &VkApi,
    api_key: Api,
    group_id: usize,
    address_id: usize,
) -> Result<u8, VkApiError> {




    let mut params = ParamGrid::new();

    let api_key = match api_key {
        Api::Flow => {
            if !api.flow_key.is_empty() {
                &api.flow_key
            } else {
                return Err(VkApiError::InternalError("You did not provide flow key".to_string()));
            }
        },
        Api::Group => {
            if !api.group_key.is_empty() {
                &api.group_key
            } else {
                return Err(VkApiError::InternalError("You did not provide group key".to_string()));
            }
        }
        Api::Service => {
            return Err(VkApiError::InternalError("Service key is not supported for this method.".to_string()));
        }
    };



    insert_params!(&mut params, group_id, address_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteAddress", API),
        api_key,
        api.v,
    )
        .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn delete_link(api: &VkApi, group_id: usize, link_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();


    insert_params!(&mut params, group_id, link_id);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}deleteLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}



pub async fn disable_online(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}disableOnline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn enable_online(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}enableOnline", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn invite(api: &VkApi, group_id: usize, user_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("user_id", user_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}invite", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn leave(api: &VkApi, group_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}leave", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn remove_user(api: &VkApi, group_id: usize, user_id: usize) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("user_id", user_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}removeUser", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}



pub async fn unban(api: &VkApi, group_id: usize, owner_id: i64) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("owner_id", owner_id.to_string());

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

pub async fn join(api: &VkApi, group_id: usize, options: JoinOptions) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    params.insert_if_not_exists(
        "not_sure",
        match options.not_sure {
            true => "1".to_string(),
            false => "0".to_string(),
        },
    );

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}join", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}

pub async fn get_online_status(
    api: &VkApi,
    group_id: usize,
) -> Result<GetOnlineStatusResponse, VkApiError> {
    let api_key = if !api.group_key.is_empty() {
        &api.group_key
    } else if !api.flow_key.is_empty() {
        &api.flow_key
    } else {
        return Err(VkApiError::InternalError("There is no existing keys in your api object. Please set one of the keys before calling this function.".to_string()));
    };

    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}getOnlineStatus", API),
        api_key,
        api.v,
    )
    .await?;

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: GetOnlineStatusResponse = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    };
}

pub async fn edit_link(
    api: &VkApi,
    group_id: usize,
    link_id: usize,
    text: String,
) -> Result<u8, VkApiError> {
    let mut params = ParamGrid::new();

    params.insert_if_not_exists("group_id", group_id.to_string());
    params.insert_if_not_exists("link_id", link_id.to_string());
    params.insert_if_not_exists("text", text);

    let response_text = send_request(
        &api.client,
        Some(params),
        &format!("{}editLink", API),
        &api.flow_key,
        api.v,
    )
    .await?;

    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    }

    Ok(1)
}
