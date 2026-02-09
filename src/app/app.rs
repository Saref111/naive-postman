use eframe::egui;
use reqwest::{blocking::Client, Method};
use std::{collections::HashMap, sync::mpsc};

use crate::app::ui::{
    handle_send_button, render_body_textarea, render_method_radio, render_response_field,
    render_url_field,
};

#[derive(Debug)]
pub struct App {
    pub url: String,
    pub method: Method,
    pub body: String,
    pub result: Option<String>,
    pub is_loading: bool,
    pub result_receiver: Option<mpsc::Receiver<String>>,
    pub client: Client,
    pub headers: Vec<(String, String)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: "https:/google.com".into(), //temp
            method: Method::GET,
            body: String::new(),
            result: None,
            is_loading: false,
            result_receiver: None,
            client: reqwest::blocking::Client::new(),
            headers: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref receiver) = self.result_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.result = Some(result);
                self.is_loading = false;
                self.result_receiver = None;
            }
        }

        if self.is_loading {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_url_field(ui, self);

            render_method_radio(ui, self);

            // let headers = self.headers.clone();
            let mut to_remove = vec![];
            for (i, (k, v)) in self.headers.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(k);
                    ui.text_edit_singleline(v);
                    if ui.button("Remove").clicked() {
                        to_remove.push(i);
                    }
                });
            }
            for i in to_remove.iter().rev() {
                self.headers.remove(*i);
            }

            let mut new_key = String::new();
            let mut new_value = String::new();
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut new_key);
                ui.text_edit_singleline(&mut new_value);
                if ui.button("Add Header").clicked() && !new_key.is_empty() {
                    self.headers.push((new_key.clone(), new_value.clone()));
                    new_key.clear();
                    new_value.clear();
                }
            });

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
                render_response_field(ui, result);
            }
        });
    }
}
