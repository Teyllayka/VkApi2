use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct VkError {
    pub error: Error,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Error {
    #[serde(alias = "error_code")]
    pub code: i32,
    #[serde(alias = "error_msg")]
    pub message: String,
    pub request_params: Vec<RequestParam>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RequestParam {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum VkApiError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    VkError(VkError),
    InternalError(String),
}

impl From<reqwest::Error> for VkApiError {
    fn from(error: reqwest::Error) -> Self {
        VkApiError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for VkApiError {
    fn from(error: serde_json::Error) -> Self {
        VkApiError::SerdeJsonError(error)
    }
}

impl fmt::Display for VkApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VkApiError::ReqwestError(error) => write!(f, "Reqwest Error: {}", error),
            VkApiError::SerdeJsonError(error) => write!(f, "Serde JSON Error: {}", error),
            VkApiError::VkError(response) => write!(f, "VK Error: {:?}", response),
            VkApiError::InternalError(response) => write!(f, "Internal Error: {:?}", response),
        }
    }
}

impl std::error::Error for VkApiError {}
