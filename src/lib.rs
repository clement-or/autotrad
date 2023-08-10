#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;

pub fn run() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        always_on_top: true,
        fullscreen: true,
        decorated: false,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "AutoTrad",
        options,
        Box::new(|_cc| Box::<app::App>::default()),
    )
}
