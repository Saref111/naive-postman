use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug, Clone)]
pub struct Headers {
    pub data: Vec<(String, String)>,
    pub new_key: String,
    pub new_value: String,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            data: vec![],
            new_key: String::new(),
            new_value: String::new(),
        }
    }

    pub fn get_map(&self) -> HeaderMap<HeaderValue> {
        let mut header_map = HeaderMap::new();

        for (k, v) in &self.data {
            if let (Ok(name), Ok(value)) = (HeaderName::from_str(k), HeaderValue::from_str(v)) {
                header_map.insert(name, value);
            }
        }

        header_map
    }
}
