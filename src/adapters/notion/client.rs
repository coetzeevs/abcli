use reqwest::header::{
    HeaderValue,
    ACCEPT,
    AUTHORIZATION,
    CONTENT_TYPE,
};

use crate::adapters::api::client;
use crate::adapters::api::models::{Header, Headers};
use crate::adapters::notion::helpers::parse_response;
use crate::app::secrets::get_secret;

use super::helpers::set_page_body;


fn set_headers() -> Headers {
    let api_key = get_secret("notion:integration_key");

    let headers = Headers {
        headers: vec![
            Header {
                header_name: ACCEPT,
                header_value: HeaderValue::from_static("application/json")
            },
            Header {
                header_name: AUTHORIZATION,
                header_value: HeaderValue::from_str(format!("Bearer {api_key}")
                    .as_str())
                    .unwrap()
            },
            Header {
                header_name: CONTENT_TYPE,
                header_value: HeaderValue::from_static("application/json")
            }
        ]
    };
    headers
}

pub async fn create(title: &String) {
    let url = format!("https://api.notion.com/v1/pages/");

    let data = set_page_body(&title);
    let response = client::post(url, set_headers(), &data);

    parse_response(response.await).await
}
