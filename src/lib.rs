use std::collections::HashMap;
use std::sync::Mutex;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::Value;
use std::str::FromStr;

pub struct LoginCaptureBrowserConfig {
    login_page_url: String,
    login_post_url: String,
    is_correct_login_check_fn: Box<dyn Fn(&str) -> bool + Sync + 'static>,
}

pub struct LoginCaptureBrowser {
    config: LoginCaptureBrowserConfig
}

struct LoginCaptureBrowserLoginResult {
    headers: HashMap<String, String>,
    response: String,
}

impl LoginCaptureBrowser {
    pub fn new(config: LoginCaptureBrowserConfig) -> Self {
        LoginCaptureBrowser {
            config
        }
    }

    pub fn run(&self) -> Result<LoginCaptureBrowserLoginResult, failure::Error> {
        // Setup channel so we can communicate with chromes request interception thread
        let (tx, rx) = std::sync::mpsc::channel();
        let tx_mutex = Mutex::new(tx);

        // Create a new chrome browser that we can control
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .headless(false)
                .build()
                .expect("Could not find chrome-executable")
        )?;

        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to(self.config.login_page_url.as_str())?;
        tab.wait_until_navigated()?;

        tab.enable_request_interception(
            &[
                headless_chrome::protocol::network::methods::RequestPattern {
                    url_pattern: Some(self.config.login_post_url.as_str()),
                    resource_type: Some("XHR"),
                    interception_stage: Some("HeadersReceived"),
                }
            ],
            Box::new(move |_transport, _session_id, event_params| {
                let post_data = event_params.request.post_data.unwrap();
                let headers = event_params.request.headers;

                let client = reqwest::Client::new();
                let mut header_map = reqwest::header::HeaderMap::new();
                for (key, value) in headers {
                    let header_name = HeaderName::from_str(key.as_str()).unwrap();
                    let header_value = HeaderValue::from_str(value.as_str()).unwrap();
                    header_map.append(header_name, header_value);
                }
                let mut response = client.post(self.config.login_post_url.as_str())
                    .headers(header_map)
                    .body(post_data)
                    .send()
                    .unwrap();
                let response_text = response.text().unwrap();
                let success = (self.config.is_correct_login_check_fn)(response_text.as_str());
                if success {
                    let mut headers = HashMap::new();
                    for (key, value) in headers {
                        headers.insert(key, value);
                    }

                    let tx = tx_mutex.lock().unwrap();
                    tx.send(LoginCaptureBrowserLoginResult {
                        response: String::from(response_text),
                        headers
                    }).unwrap();
                }
                headless_chrome::browser::tab::RequestInterceptionDecision::Continue
            }),
        )?;
        let result = rx.recv().unwrap();

        Ok(result)
    }
}