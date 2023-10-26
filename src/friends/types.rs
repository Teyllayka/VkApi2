pub struct AddOptions {
    pub text: String,
    pub follow: bool,
}

impl Default for AddOptions {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            follow: false,
        }
    }
}
