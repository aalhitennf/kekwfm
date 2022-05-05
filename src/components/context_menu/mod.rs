use eframe::egui::{self, Align, Direction};

use kekwlib::dirutils::DirectoryListingItem;

use crate::eevertti::{send_event, KekEvent};

const BUTTON_SIZE: [f32; 2] = [160.0, 25.0];

pub fn file_right_click(ui: &mut egui::Ui, item: &DirectoryListingItem) {
    ui.with_layout(
        egui::Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::LEFT),
        |ui| {
            if create_button(ui, &item.filename).clicked() {
                ui.close_menu();
            }

            if create_button(ui, "Open").clicked() {
                if item.is_dir {
                    send_event(KekEvent::Navigate(item.path.clone()));
                } else if item.is_file {
                    send_event(KekEvent::XdgOpenFile(item.path.clone()));
                }

                ui.close_menu();
            };

            if create_button(ui, "Trash").clicked() {
                send_event(KekEvent::TrashFile(item.path.clone()));
                ui.close_menu();
            }

            if create_button(ui, "Trash selected file(s)").clicked() {
                send_event(KekEvent::TrashSelected);
                ui.close_menu();
            }

            if item.is_dir {
                if create_button(ui, "Favorite").clicked() {
                    send_event(KekEvent::FavouriteFolder(item.clone()));
                    ui.close_menu();
                }
            }
        },
    );
}

fn create_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add_sized(BUTTON_SIZE, egui::Button::new(text).frame(false))
}
