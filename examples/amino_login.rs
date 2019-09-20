use chrome_login_capture::{LoginCaptureBrowser, LoginCaptureBrowserConfig};
use serde_json::Value;

fn main() {
    let browser = LoginCaptureBrowser::new(LoginCaptureBrowserConfig {
        login_page_url: String::from("https://www.aminoapps.com"),
        login_post_url: String::from("https://www.aminoapps.com/api/auth"),
        is_correct_login_check_fn: Box::new(|response_text| {
            let data: Value = serde_json::from_str(response_text.as_str()).unwrap();
            let success = data["result"]["nickname"].is_string();
            success
        })
    });
    let _result = browser.run();
}