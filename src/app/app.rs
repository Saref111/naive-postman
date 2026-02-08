use eframe::egui;
use reqwest::Method;
use std::sync::mpsc;

use crate::{
    app::ui::{handle_send_button, render_body_textarea, render_method_radio, render_url_field},
    utils::send_req,
};

#[derive(Debug)]
pub struct App {
    pub url: String,
    pub method: Method,
    pub body: String,
    pub result: Option<String>,
    pub is_loading: bool,
    pub result_receiver: Option<mpsc::Receiver<String>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: Method::GET,
            body: String::new(),
            result: None,
            is_loading: false,
            result_receiver: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Check if we have a result from the background thread
        if let Some(ref receiver) = self.result_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.result = Some(result);
                self.is_loading = false;
                self.result_receiver = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_url_field(ui, self);

            render_method_radio(ui, self);

            if self.method == Method::POST {
                render_body_textarea(ui, self);
            }

            if !self.is_loading {
                let send_buttons = ui.button("SEND");
                if send_buttons.clicked() {
                    handle_send_button(self);
                }
            } else {
                ui.add(egui::Spinner::new());
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
