use reqwest::{blocking::Client, Method};

use crate::app::headers::Headers;

pub struct RequestData {
    pub url: String,
    pub method: Method,
    pub body: String,
    pub client: Client,
    pub headers: Headers,
}
