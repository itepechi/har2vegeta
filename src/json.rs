use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Json {
    pub method: String,
    pub url: String,
    pub header: HashMap<String, Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
