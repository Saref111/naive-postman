use reqwest::{blocking::Response, Method};

use crate::app::MyApp;

pub fn send_req(app: &MyApp) -> String {
    let client = reqwest::blocking::Client::new();
    let request = match app.method {
        Method::GET => client.get(&app.url),
        Method::POST => client.post(&app.url).body(app.body.clone()),
        _ => return "Unsupported method".to_string(),
    };
    match request.send() {
        Ok(resp) => parse_resp(resp),
        Err(e) => format!("Request failed: {}", e),
    }
}

pub fn parse_resp(resp: Response) -> String {
    let status = resp.status();
    let headers = resp.headers().clone();
    match resp.text() {
        Ok(body) => {
            let mut result = format!(
                "HTTP/1.1 {} {}\n",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown")
            );
            for (key, value) in headers.iter() {
                result.push_str(&format!(
                    "{}: {}\n",
                    key,
                    value.to_str().unwrap_or("Invalid header")
                ));
            }
            result.push_str("\n");
            result.push_str(&body);
            result
        }
        Err(e) => format!("Error reading response body: {}", e),
    }
}
