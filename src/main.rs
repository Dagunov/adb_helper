#![windows_subsystem = "windows"]

use adb_helper::App;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.resizable = false;
    native_options.initial_window_size = Some(eframe::egui::vec2(400.0, 400.0));
    eframe::run_native(
        "adb-helper",
        native_options,
        Box::new(|cc| Box::new(App::new(cc)))
    )
}