use eframe::{run_native, App, Error, Frame, NativeOptions};
use egui::Context;

struct Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        println!("no witam witam");
    }
}

fn main() -> Result<(), Error> {
    let win_option = NativeOptions::default();

    // Closure zwracająca Result z aplikacją
    run_native(
        "nowa",
        win_option,
        Box::new(|_cc| Ok(Box::new(Headlines {}))),
    )?;

    Ok(())
}
