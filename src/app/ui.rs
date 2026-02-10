use eframe::egui::{self, Ui};
use reqwest::Method;
use std::sync::mpsc;
use std::thread;

use crate::app::headers::{self, Headers};
use crate::app::{App, RequestData};
use crate::utils::send_req;

pub fn render_url_field(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        let name_label = ui.label("URL: ");
        ui.text_edit_singleline(&mut app.url)
            .labelled_by(name_label.id);
    });
}

pub fn render_method_radio(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        ui.radio_value(&mut app.method, Method::GET, "GET");
        ui.radio_value(&mut app.method, Method::POST, "POST");
    });
}

pub fn render_body_textarea(ui: &mut Ui, app: &mut App) {
    let label = ui.label("Body: ");
    ui.text_edit_multiline(&mut app.body).labelled_by(label.id);
}

pub fn render_response_field(ui: &mut Ui, result: &String) {
    ui.separator();
    ui.label("Response:");

    let (head, body) = result.split_once("\n\n").unwrap_or((result, ""));

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(350.0)
        .show(ui, |ui| {
            ui.add(egui::Label::new(egui::RichText::new(head).monospace()).selectable(true));
            if !body.is_empty() {
                ui.separator();
                ui.add(egui::Label::new(body).selectable(true));
            }
        });
}

pub fn render_headers(ui: &mut Ui, headers: &mut Headers) {
    let mut to_remove = vec![];
    for (i, (k, v)) in headers.data.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(k);
            ui.text_edit_singleline(v);
            if ui.button("Remove").clicked() {
                to_remove.push(i);
            }
        });
    }
    for i in to_remove.iter().rev() {
        headers.data.remove(*i);
    }

    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut headers.new_key);
        ui.text_edit_singleline(&mut headers.new_value);
        if ui.button("Add Header").clicked() && !headers.new_key.is_empty() {
            headers
                .data
                .push((headers.new_key.clone(), headers.new_value.clone()));
            headers.new_key.clear();
            headers.new_value.clear();
        }
    });
}

pub fn handle_send_button(app: &mut App) {
    if !app.url.is_empty() {
        app.is_loading = true;
        let url = app.url.clone();
        let method = app.method.clone();
        let body = app.body.clone();
        let client = app.client.clone();
        let headers = app.headers.clone();
        let (tx, rx) = mpsc::channel();
        app.result_receiver = Some(rx);
        thread::spawn(move || {
            let rd = RequestData {
                url,
                method,
                body,
                client,
                headers,
            };
            let result = send_req(rd);
            tx.send(result).unwrap();
        });
    } else {
        app.result = Some("Please enter a URL".to_string());
    }
}
