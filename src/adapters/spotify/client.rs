// use core::result::Result::Ok;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::adapters::api::client::client;
use crate::adapters::api::models::{Header, Headers};
use crate::adapters::spotify::helpers::parse_response;


// TODO: finish
fn set_auth() {}

fn set_headers() -> Headers {
    let fake_auth = "";
    let headers = Headers {
        headers: vec![
            Header {
                header_name: ACCEPT,
                header_value: HeaderValue::from_static("application/json")
            },
            Header {
                header_name: AUTHORIZATION,
                header_value: HeaderValue::from_str(format!("Bearer {}", fake_auth)
                    .as_str())
                    .unwrap()
            },
            Header {
                header_name: CONTENT_TYPE,
                header_value: HeaderValue::from_static("application/json")
            },
        ]
    };
    headers
}

pub async fn query(query: &String) {

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = query
    );
    
    let response = client(url, set_headers());

    parse_response(response.await).await;
}

// TODO: discuss error variants https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html
