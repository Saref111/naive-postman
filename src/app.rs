use eframe::egui;
use reqwest::Method;

use crate::utils::send_req;

#[derive(Debug)]
pub struct MyApp {
    pub url: String,
    pub method: Method,
    pub body: String,
    pub result: Option<String>,
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
