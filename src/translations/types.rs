use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResult {
    pub texts: Vec<String>,
    pub source_lang: String,
}
