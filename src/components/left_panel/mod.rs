use eframe::egui;

use crate::app::KekwFM;

pub struct LeftPanel {}

pub fn build(ctx: &egui::Context, _app: &mut KekwFM) {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Locations");
            ui.label("Home");
            ui.label("Root");
            ui.label("Config");
            ui.label("Devices");
            ui.label("SSD 128 Gb");
            ui.label("Usb 16 Gb");
        });
}
