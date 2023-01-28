use reqwest::header::HeaderName;
use crate::adapters::api::helpers::construct_headers;

#[tokio::main]
pub async fn client(url: String, headers: std::collections::HashMap<HeaderName, &'static String>) -> reqwest::Response {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .headers(construct_headers(headers))
        .send()
        .await
        .unwrap();

    return response;
}
