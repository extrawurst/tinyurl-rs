use crate::{
    error::Result,
    types::{CreateRequest, CreateResponse},
};
use async_trait::async_trait;
use reqwest::Url;

#[async_trait]
pub trait TinyUrlOpenAPI {
    async fn create(&self, request: CreateRequest) -> Result<CreateResponse>;
}

#[derive(Default, Debug)]
pub struct TinyUrlAPI {
    pub token: String,
}

const BASE_URL: &str = "https://api.tinyurl.com";

#[async_trait]
impl TinyUrlOpenAPI for TinyUrlAPI {
    async fn create(&self, request: CreateRequest) -> Result<CreateResponse> {
        let url = format!("{}/create?api_token={}", BASE_URL, self.token);

        let url: Url = url.parse()?;
        let client = reqwest::Client::new();

        let resp = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<CreateResponse>()
            .await?;

        Ok(resp)
    }
}
