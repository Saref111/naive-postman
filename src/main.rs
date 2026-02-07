use std::default;

use eframe::egui;

#[derive(PartialEq)]
enum Method {
    POST,
    GET,
}

struct MyApp {
    url: String,
    method: Method,
    body: Option<()>,
    result: (),
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: Method::GET,
            body: None,
            result: (),
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 240.0]),
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
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
