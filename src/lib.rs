mod error;
use error::VkApiError;
use reqwest::Client;
pub mod account;
pub mod friends;
pub mod messages;
mod param_grid;
pub mod status;
pub use param_grid::ParamGrid;
pub mod calls;
pub mod donut;
pub mod fave;
pub mod notes;
pub mod photos;
pub mod users;
pub mod utils;

pub mod gifts;
pub mod likes;
pub mod podcasts;
pub mod translations;

pub mod wall;

pub mod docs;

pub mod groups;

pub mod stories;

pub mod market;

pub mod apps;
pub mod streaming;

pub mod orders;

pub mod board;

pub enum Api {
    Service,
    Group,
    Flow,
}

pub struct VkApi {
    service_key: String,
    group_key: String,
    flow_key: String,
    client: Client,
    v: f32,
}

impl VkApi {
    pub fn new(
        service_key: Option<String>,
        group_key: Option<String>,
        flow_key: Option<String>,
        v: Option<f32>,
    ) -> Self {
        let service_key = match service_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let group_key = match group_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let flow_key = match flow_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let v = v.unwrap_or_else(|| 5.199);

        Self {
            service_key,
            group_key,
            flow_key,
            client: Client::new(),
            v,
        }
    }

    pub fn get_group_key(self) -> Option<String> {
        return if self.group_key == "" {
            None
        } else {
            Some(self.group_key)
        };
    }

    pub fn get_service_key(self) -> Option<String> {
        return if self.service_key == "" {
            None
        } else {
            Some(self.service_key)
        };
    }

    pub fn get_flow_key(self) -> Option<String> {
        return if self.flow_key == "" {
            None
        } else {
            Some(self.flow_key)
        };
    }

    pub fn get_version(self) -> f32 {
        return self.v;
    }

    pub fn set_group_key(mut self, group_key: String) {
        self.group_key = group_key;
    }

    pub fn set_service_key(mut self, service_key: String) {
        self.service_key = service_key;
    }

    pub fn set_flow_key(mut self, flow_key: String) {
        self.flow_key = flow_key;
    }

    pub fn set_version(mut self, v: f32) {
        self.v = v;
    }
}

async fn send_request(
    client: &Client,
    params: Option<ParamGrid>,
    url: &str,
    key: &str,
    v: f32,
) -> Result<String, VkApiError> {
    let mut params = params.unwrap_or_default();

    params.insert_if_not_exists("v", v);

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .form(&params.data)
        .send()
        .await?;

    let response_text = response.text().await?;

    Ok(response_text)
}

#[macro_export]
macro_rules! insert_params {
    ($params:expr, $($value:expr),* $(,)?) => {
        $(
            $params.insert_if_not_exists(stringify!($value), $value.to_string());
        )*
    };
}


#[cfg(test)]
mod tests {
    use crate::status::methods::get;
    use crate::users::methods::get_followers;
    use crate::users::types::UserGetFollowersOptions;
    use crate::users::types::{Fields, User};
    use crate::streaming::methods::{get_settings, get_stem};
    use crate::utils::methods::get_link_stats;
    use crate::utils::types::{GetLinkStatsOptions, Interval};
    use crate::VkApi;
    use dotenvy::dotenv;
    use crate::account::methods::get_info;
    use crate::account::types::GetInfoOptions;
    use crate::groups::methods::add_address;

    #[tokio::test]
    async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect(".env file not found");

        //get env keys
        let service_key = dotenvy::var("SERVICE_KEY").unwrap();
        let group_key = dotenvy::var("GROUP_KEY").unwrap();
        let flow_key = dotenvy::var("FLOW_KEY").unwrap();

        //create api with these keys
        let api = VkApi::new(
            Some(service_key),
            Some(group_key),
            Some(flow_key),
            Some(5.199),
        );


        // let v2 = get_settings(&api).await?;
        //
        // println!("{:?}", v2);


        let v = add_address(&api, 222943017, "test".to_string(), "test".to_string(), 1, 1, 1, 1, None).await?;

        println!("{:?}", v);






        assert!(true);

        Ok(())
    }
}
