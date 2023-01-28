use reqwest::header::{HeaderMap, HeaderName, HeaderValue};


pub fn construct_headers(headers: std::collections::HashMap<HeaderName, &'static String>) -> HeaderMap {
    let mut h_map = HeaderMap::new();

    for (k, v) in headers {
        h_map.entry(k).or_insert_with(|| { HeaderValue::from_static(v.as_str()) });
    }
    h_map
}
