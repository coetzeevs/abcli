use crate::adapters::api::helpers::construct_headers;
use crate::adapters::api::models::Headers;

pub async fn client(url: String, headers: Headers) -> reqwest::Response {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .headers(construct_headers(headers))
        .send()
        .await
        .unwrap();

    response
}
