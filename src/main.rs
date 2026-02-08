mod app;
mod utils;
use eframe::egui;

use crate::app::MyApp;

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
