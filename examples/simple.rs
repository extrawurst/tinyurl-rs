use std::env;
use tinyurl_rs::{CreateRequest, Result, TinyUrlAPI, TinyUrlOpenAPI};

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("TOKEN").unwrap_or_default();
    let api = TinyUrlAPI { token };

    let res = api
        .create(CreateRequest::new(String::from("http://beta.live-ask.com")))
        .await?;

    println!("result:\n{:?}", res);

    Ok(())
}
