use std::path::Path;

use reqwest::{Response, StatusCode};
use tracing::info;

use super::models::page;

pub fn set_page_body(title: String, database_id: String) -> page::Request {
    let path = Path::new("./templates/notion/pages/daily.json");
    let mut text = std::fs::read_to_string(path).unwrap();
    text = text.replace("var.date_today", title.as_str());
    text = text.replace("var.database_id", database_id.as_str());
    let p: page::Request = serde_json::from_str(&text).unwrap();
    p
}

pub async fn parse_response(response: Response) {
    match response.status() {
        StatusCode::OK => {
            match response.json::<page::Response>().await {
                Ok(parsed) => parse_success(parsed),
                Err(_) => panic!("omg panic"),
            };
        }
        StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {other:#?}");
        }
    };
}

pub fn parse_success(obj: page::Response) {
    info!("Successfully created new page(s)!");

    for title in obj.properties.name.title {
        info!("Title: {}", title.plain_text)
    }
}
