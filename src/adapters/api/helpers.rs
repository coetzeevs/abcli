use reqwest::header::HeaderMap;
use crate::adapters::api::models::Headers;


pub fn construct_headers(headers: Headers) -> HeaderMap {
    let mut hmap = HeaderMap::new();

    for i in headers.headers {
        hmap.entry(i.header_name).or_insert_with(|| {i.header_value});
    }

    hmap
}
