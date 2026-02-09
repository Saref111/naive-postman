#[derive(Debug)]
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
}
