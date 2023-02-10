use crate::adapters::api::helpers::construct_headers;
use crate::adapters::api::models::Headers;
use crate::adapters::notion::models::page::Request;


pub async fn post(url: String, headers: Headers, body: &Request) -> reqwest::Response {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(construct_headers(headers))
        .header("Notion-Version", "2022-06-28")
        .json(body)
        .send()
        .await
        .unwrap();

    response
}

// TODO: this needs a lot of cleanup. This is gross..
