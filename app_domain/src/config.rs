use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub apis: Vec<ApiReverse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiReverse {
    pub endpoint: String,
    pub backend: Backend,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backend {
    pub url_pattern: String,
    pub host: Vec<String>,
}
