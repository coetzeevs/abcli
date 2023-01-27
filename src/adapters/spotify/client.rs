// use core::result::Result::Ok;
use reqwest::StatusCode;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::adapters::spotify::models::APIResponse;
// use crate::adapters::spotify::helpers::print_tracks;

#[tokio::main]
pub async fn query(query: &String) -> Result<(), Box<dyn std::error::Error>> {
    let auth_token = &String::from("BQAQRQfVlEOChnH8-KD3Y3nbl842lgx3rggdJ0Orb95i-4WrTdZy3JD8HF3V-O3cJqk5EQgrGEZg2S-fIL8HyfKZ738V8hoTr8m93NNBf_paHDF4hHAMSXJnNyM_s4ti4D9dUTVYW0aq9mof-L37Mi_lQRi_4pT05QcMIKaJZE716yU4ZW39lPghhSEcqO_OOmMH");
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = query
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
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
    //       implement base client
    //       add more commands and parameters
    Ok(())
}

// TODO: discuss error variants https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html