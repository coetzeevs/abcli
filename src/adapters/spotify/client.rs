// use core::result::Result::Ok;
use reqwest::StatusCode;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::adapters::api::client::client;
use crate::adapters::api::models::{Header, Headers};
use crate::adapters::spotify::models::APIResponse;


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

pub async fn query(query: &String) -> Result<(), Box<dyn std::error::Error>> {

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = query
    );
    
    let response = client(url, set_headers());

    match response.status() {
        StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("{:#?}", parsed),
                Err(_) => panic!("Err caught on status OK...")
            };
        }
        StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
    // TODO: cleanup response
    //       add more commands and parameters
    Ok(())
}

// TODO: discuss error variants https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html