use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Form {
    pub form: HashMap<String, String>,
    pub(crate) target_uri: String,
}

impl Form {
    pub fn new(form: HashMap<String, String>, target_uri: &str) -> Self {
        Form {
            form,
            target_uri: target_uri.to_string(),
        }
    }
}
