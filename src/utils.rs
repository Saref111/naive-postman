use reqwest::{blocking::Client, blocking::Response, Method};

pub fn send_req(url: &str, method: Method, body: &str, client: &Client) -> String {
    let request = match method {
        Method::GET => client.get(url),
        Method::POST => client.post(url).body(body.to_string()),
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
