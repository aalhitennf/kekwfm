use eframe::{
    egui::{self, Layout, PointerButton, Sense},
    emath::{Vec2, Align},
    epaint::TextureId,
};
use egui_extras::{Size, TableBuilder};

use kekwlib::dirutils::{DirectoryListingItem, ReadDirOptions};

use crate::{
    components::context_menu::file_right_click,
    eevertti::{send_event, KekEvent},
    textures::Textures,
};

const MIN_WIDTH: f32 = 100.0; 

#[derive(Default)]
pub struct GridView {
    pub all_selected: bool,
}

impl GridView {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        items: &[DirectoryListingItem],
        textures: &Textures,
        _read_dir_options: &mut ReadDirOptions,
    ) {
        let n_column_amount =  std::cmp::max(1, ((ui.available_width() / MIN_WIDTH).round() - 1.0) as usize);
        let column_amount = std::cmp::min(n_column_amount, items.len());

        let total_rows = items.len() / column_amount + 1;

        TableBuilder::new(ui).clip(false)
            .columns(Size::exact(MIN_WIDTH), column_amount)
            .column(Size::remainder())
            .resizable(false)
            .body(|body| {
                body.rows(MIN_WIDTH, total_rows, |index, mut row| {
                    for i in 0..column_amount {
                        if let Some(item) = items.get((index * column_amount) + i) {
                            let icon = if item.is_dir { &textures.folder_64 } else { &textures.file_64 };
                            row.col(|ui| {
                                row_file_item(ui, item, icon.id());
                            });
                        }
                    }
                    row.col(|_| {});
                });
            });
    }
}

fn row_file_item(ui: &mut egui::Ui, item: &DirectoryListingItem, icon: TextureId) {
    let ok = ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
        let k = ui.add(egui::ImageButton::new(icon, Vec2::new(60.0, 60.0)).frame(false).sense(Sense::click()));
        let o = ui.add(egui::Label::new(item.filename.to_string()).sense(Sense::click())).context_menu(|ui| file_right_click(ui, item));
        (k, o)
    });

    if ok.inner.0.double_clicked_by(PointerButton::Primary) || ok.inner.1.double_clicked_by(PointerButton::Primary) {
        if item.is_dir {
            send_event(KekEvent::Navigate(item.path.clone()));
        } else if item.is_file {
            send_event(KekEvent::XdgOpenFile(item.path.clone()));
        }
    }
}

// fn bytes_to_human_readable(bytes: u64) -> String {
//     let bytes = bytes as f64;
//     match bytes {
//         x if x < 1024.0 => format!("{} bytes", bytes),
//         x if x < 1_048_576.0 => format!("{:.1} KiB", bytes / 1024.00),
//         x if x < 1_073_741_824.0 => format!("{:.2} MiB", bytes / 1024.00 / 1024.00),
//         x if x < 1_099_511_627_776.0 => format!("{:.2} GiB", bytes / 1024.00 / 1024.00 / 1024.00),
//         x if x < 1_125_899_906_842_624.0 => {
//             format!("{:.2} TiB", bytes / 1024.00 / 1024.00 / 1024.00 / 1024.00)
//         }
//         _ => String::from("Very large"),
//     }
// }
