#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
pub mod components;
pub mod eevertti;
pub mod rdev_events;
pub mod textures;

use app::KekwFM;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_pos: Some(eframe::egui::pos2(1600.0, 500.0)),
        initial_window_size: Some(eframe::egui::vec2(900.0, 500.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native("KekwFM", options, Box::new(|cc| Box::new(KekwFM::new(cc))));
}
