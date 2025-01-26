mod gui;
use gui::gui::MyApp;
mod imgUtils;
mod file_operations;

use eframe::NativeOptions;

const APP_NAME: &str = "Menadżer grafiki";

fn main() -> Result<(), eframe::Error> {
        eframe::run_native(
        APP_NAME,
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}