use serde::{Deserialize, Serialize};

// see https://tinyurl.com/app/dev

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRequest {
    pub url: String,
    pub domain: Option<String>,
    pub alias: Option<String>,
    pub tags: Option<String>,
    pub expires_at: Option<String>,
}

impl CreateRequest {
    pub fn new(url: String) -> Self {
        Self {
            url,
            domain: None,
            alias: None,
            tags: None,
            expires_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlData {
    pub domain: String,
    pub alias: String,
    pub deleted: bool,
    pub archived: bool,
    pub tiny_url: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub code: i32,
    pub errors: Vec<String>,
    pub data: Option<UrlData>,
}
