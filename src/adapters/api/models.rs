use reqwest::header::{HeaderName, HeaderValue};

pub struct Headers {
    pub headers: Vec<Header>,
}

pub struct Header {
    pub header_name: HeaderName,
    pub header_value: HeaderValue,
}
