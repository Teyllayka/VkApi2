pub struct AddDocsOptions {
    pub access_key: String,
}

impl Default for AddDocsOptions {
    fn default() -> Self {
        Self {
            access_key: "".to_string(),
        }
    }
}
