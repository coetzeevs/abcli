use reqwest::{StatusCode, Response};
use tracing::info;

use super::models::page;
use super::models::properties;
use super::models::shared;

pub fn set_annontations() -> shared::Annotations {
    let annote = shared::Annotations {
        bold: false,
        italic: false,
        strikethrough: false,
        underline: false,
        code: false,
        color: "default".to_string()
    };
    annote
}

pub fn set_page_body(title: &String) -> page::Request {

    let pt = title.to_owned();

    let parent = shared::Parent::DatabaseParent(shared::DatabaseParent {
        type_field: "database_id".to_string(),
        database_id: "a58d647fc64840f39ab11fbff376884d".to_string() // TODO: set from envars
    });
    let properties = properties::Properties {
        name: Some(properties::Name {
            id: "LKBS985".to_string(),
            type_field: "title".to_string(),
            title: vec![shared::Title {
                type_field: "text".to_string(),
                text: shared::Text {
                    content: pt.clone(),
                    link: None
                },
                annotations: Some(set_annontations()),
                plain_text: pt,
                href: None
            }]
        })
    };
    let root = page::Request {
        parent,
        properties,
    };

    root
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
    // let serialized_data = serde_json::to_string(obj).unwrap();
    info!("Successfully created new page!\n");

    for title in obj.properties.name.unwrap().title {
        info!("Title: {}\n", title.plain_text)
    };
}