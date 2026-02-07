use eframe::egui;
use reqwest::{blocking::Response, Method};

#[derive(Debug)]
struct MyApp {
    url: String,
    method: Method,
    body: String,
    result: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: Method::GET,
            body: String::new(),
            result: None,
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1120.0, 740.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PNaive",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("URL: ");
                ui.text_edit_singleline(&mut self.url)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.method, Method::GET, "GET");
                ui.radio_value(&mut self.method, Method::POST, "POST");
            });

            if self.method == Method::POST {
                let label = ui.label("Body: ");
                ui.text_edit_multiline(&mut self.body).labelled_by(label.id);
            }
            if ui.button("SEND").clicked() {
                if !self.url.is_empty() {
                    self.result = Some(send_req(&self));
                } else {
                    self.result = Some("Please enter a URL".to_string());
                }
            }

            if let Some(result) = &self.result {
                ui.separator();
                ui.label("Response:");
                let mut result_text = result.clone();
                ui.add(
                    egui::TextEdit::multiline(&mut result_text)
                        .interactive(false)
                        .desired_rows(10),
                );
            }
        });
    }
}

fn send_req(app: &MyApp) -> String {
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

fn parse_resp(resp: Response) -> String {
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
