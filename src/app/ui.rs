use eframe::egui::Ui;
use reqwest::Method;
use std::sync::mpsc;
use std::thread;

use crate::app::App;
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

pub fn handle_send_button(app: &mut App) {
    if !app.url.is_empty() {
        app.is_loading = true;
        let url = app.url.clone();
        let method = app.method.clone();
        let body = app.body.clone();
        let (tx, rx) = mpsc::channel();
        app.result_receiver = Some(rx);
        thread::spawn(move || {
            let result = send_req(&url, method, &body);
            tx.send(result).unwrap();
        });
    } else {
        app.result = Some("Please enter a URL".to_string());
    }
}
