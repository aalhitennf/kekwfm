use eframe::{egui::{self, Sense, TextStyle}, emath::vec2};
use egui_extras::{TableBuilder, Size};

use kekwlib::dirutils::{DirectoryListingItem, FileSorting};

use crate::app::KekwFM;

const ICON_MARGIN: f32 = 4.0;
const HEADER_SIZE: f32 = 20.0;

pub struct ListView;

fn clickable_label(text: &str) -> egui::Label {
    egui::Label::new(text).sense(Sense::click())
}

impl ListView {
    pub fn build(ui: &mut egui::Ui, app: &mut KekwFM, items: Vec<DirectoryListingItem>) {
        let text_height = TextStyle::Body.resolve(ui.style()).size + 10.0;
        let icon_size = vec2(text_height - ICON_MARGIN, text_height - ICON_MARGIN);

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(450.0).at_least(200.0))
            .column(Size::relative(0.1))
            .column(Size::remainder())
            .resizable(true)
            .header(HEADER_SIZE, |mut header| {
                header.col(|ui| {
                    // ui.with_layout(Lay, add_contents)
                    if ui.add(clickable_label("Filename")).clicked() {
                        if app.read_dir_options.sorting == FileSorting::Alphabetical {
                            app.read_dir_options.reverse = !app.read_dir_options.reverse;
                        } else {
                            app.read_dir_options.sorting = FileSorting::Alphabetical;
                        }
                        app.refresh_current_dir_listing();
                    }
                });
                header.col(|ui| {
                    if ui.add(clickable_label("Type")).clicked() {
                        if app.read_dir_options.sorting == FileSorting::Extension {
                            app.read_dir_options.reverse = !app.read_dir_options.reverse;
                        } else {
                            app.read_dir_options.sorting = FileSorting::Extension;
                        }
                        app.refresh_current_dir_listing();
                    }
                });
                header.col(|ui| {
                    if ui.add(clickable_label("Size")).clicked() {
                        if app.read_dir_options.sorting == FileSorting::Size {
                            app.read_dir_options.reverse = !app.read_dir_options.reverse;
                        } else {
                            app.read_dir_options.sorting = FileSorting::Size;
                        }
                        app.refresh_current_dir_listing();
                    }
                });
            })
            .body(|body| {
                body.rows(text_height, app.directory_listing.items.len(), |index, mut row| {
                    if let Some(i) = items.get(index) {
                        row.col(|ui| {

                            if i.is_dir {
                                ui.add(egui::Image::new(app.textures.folder.id(), icon_size));
                            } else {
                                ui.add(egui::Image::new(app.textures.file.id(), icon_size));
                            }

                            if ui.add(clickable_label(&i.filename)).double_clicked() {
                                let p = i.path.clone();
                                app.try_navigate(Some(p));
                            }
                        });
                        row.col(|ui| {
                            ui.label(&i.extension);
                        });
                        row.col(|ui| {
                            ui.label(bytes_to_human_readable(i.size_bytes));
                        });
                    }

                });
            });
    }
}

fn bytes_to_human_readable(bytes: u64) -> String{
    let bytes = bytes as f64;
    match bytes {
        x if x < 1024.0 => format!("{} bytes", bytes),
        x if x < 1_048_576.0 => format!("{:.1} KiB", bytes / 1024.00),
        x if x < 1_073_741_824.0 => format!("{:.2} MiB", bytes / 1024.00 / 1024.00),
        x if x < 1_099_511_627_776.0 => format!("{:.2} GiB", bytes / 1024.00 / 1024.00 / 1024.00),
        x if x < 1_125_899_906_842_624.0 => format!("{:.2} TiB", bytes / 1024.00 / 1024.00 / 1024.00 / 1024.00),
        _ => String::from("Very large"),
    }
}