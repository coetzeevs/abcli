use reqwest::header::{
    HeaderValue,
    ACCEPT,
    AUTHORIZATION,
    CONTENT_TYPE
};

use crate::adapters::api::client::client;
use crate::adapters::api::models::{Header, Headers};
use crate::adapters::spotify::helpers::parse_response;
use crate::app::secrets::get_secret;


// TODO: finish | need to make this generic as well
    // Add support for:
    // Basic
    // OAuth
    // Bearer

fn set_headers() -> Headers {
    let api_key = get_secret("spotify:api_key");
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
            },
        ]
    };
    headers
}

pub async fn query(query: &String) {

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track,artist");
    
    let response = client(url, set_headers());

    parse_response(response.await).await
}

// TODO: Can clean this up out of the code. Won't use right now and can add it back in when ready
