use reqwest::{StatusCode, Response};
use tracing::info;

use super::models::page;
use super::models::properties;
use super::models::shared;


pub fn set_annontations() -> shared::Annotations {
    shared::Annotations {
        bold: false,
        italic: false,
        strikethrough: false,
        underline: false,
        code: false,
        color: "default".to_string()
    }
}

pub fn set_page_body(title: String, database_id: String) -> page::Request {

    let parent = shared::Parent::Database(shared::DatabaseParent {
        type_field: "database_id".to_string(),
        database_id
    });
    let properties = properties::Properties {
        name: Some(properties::Name {
            id: "LKBS985".to_string(),
            type_field: "title".to_string(),
            title: vec![shared::Title {
                type_field: "text".to_string(),
                text: shared::Text {
                    content: title.clone(),
                    link: None
                },
                annotations: Some(set_annontations()),
                plain_text: title,
                href: None
            }]
        })
    };
    page::Request {
        parent,
        properties,
    }
}

pub async fn parse_response(response: Response) {

    match response.status() {
        StatusCode::OK => {
            match response.json::<page::Response>().await {
                Ok(parsed) => parse_success(parsed),
                Err(_) => panic!("omg panic")
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

    for title in obj.properties.name.unwrap().title {
        info!("Title: {}", title.plain_text)
    };
}
