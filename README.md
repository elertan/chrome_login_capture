# Chrome Login Capture

Opens a controlled chrome instance that allows the controller to capture e.g. login tokens.

![sample_image](https://imgur.com/jIYbJUK.jpg)

A good use case might be when a website implements a captcha strategy on their login page.
Using a controlled chrome instance the captcha will validate because a user is logging in manually, but it still allows
us to grab their access token/cookies.

Example (taken from examples/amino_login):
```rust
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
// Result contains headers, cookies and the successful response from the login_post_url
```
