#[macro_use]
extern crate serde_derive;

use chrome_login_capture::{LoginCaptureBrowser, LoginCaptureBrowserConfig};
use serde_json::Value;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

#[derive(Serialize)]
struct PostData {
    #[allow(non_snake_case)]
    ndcIds: [&'static str; 4]
}

fn main() -> Result<(), failure::Error>{
    let browser = LoginCaptureBrowser::new(LoginCaptureBrowserConfig {
        login_page_url: String::from("https://www.aminoapps.com"),
        login_post_url: String::from("https://aminoapps.com/api/auth"),
        is_correct_login_check_fn: &|response_text| {
            dbg!(response_text);
            let data: Value = serde_json::from_str(response_text).unwrap();
            let success = data["result"]["nickname"].is_string();
            success
        }
    });
    let result = browser.run()?;
    dbg!(&result);

    let mut headermap = HeaderMap::new();
    for (key, value) in &result.headers {
        headermap.insert(HeaderName::from_str(key.as_str()).unwrap(), HeaderValue::from_str(value.as_str()).unwrap());
    }

    let body = PostData {
        ndcIds: ["1", "2", "3", "4"]
    };
    let body_str = serde_json::to_string(&body).unwrap();

    // Fetch protected resource using acquired credentials
    let client = reqwest::Client::new();
    let mut response = client.post("https://aminoapps.com/api/global-chat-communities")
        .headers(headermap)
        .body(body_str)
        .send()
        .unwrap();
    dbg!(&response.text());
    Ok(())
}