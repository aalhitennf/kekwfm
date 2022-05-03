use eframe::{
    egui::{self, Sense, TextStyle},
    emath::vec2,
};
use egui_extras::{Size, TableBuilder};

use kekwlib::dirutils::{FileSorting, ReadDirOptions, DirectoryListing, DirectoryListingItem};

use crate::{
    components::context_menu::file_right_click,
    eevertti::{send_event, KekEvent}, textures::TextureLoader,
};

const ICON_MARGIN: f32 = 4.0;
const HEADER_SIZE: f32 = 20.0;


fn clickable_label(text: &str) -> egui::Label {
    egui::Label::new(text).sense(Sense::click())
}

pub trait DirectoryView {
    fn show(&mut self, ui: &mut egui::Ui, read_dir_options: &mut ReadDirOptions, textures: &TextureLoader, items: &DirectoryListing);
}

#[derive(Default)]
pub struct ListView;


impl ListView {
    pub fn show(&mut self, ui: &mut egui::Ui, items: &[DirectoryListingItem], textures: &TextureLoader, read_dir_options: &mut ReadDirOptions) {
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
                    if ui.add(clickable_label("Filename")).clicked() {
                        if read_dir_options.sorting == FileSorting::Alphabetical {
                            read_dir_options.reverse = !read_dir_options.reverse;
                        } else {
                            read_dir_options.sorting = FileSorting::Alphabetical;
                        }
                        send_event(KekEvent::RefreshDirList);
                    }
                });
                header.col(|ui| {
                    if ui.add(clickable_label("Type")).clicked() {
                        if read_dir_options.sorting == FileSorting::Extension {
                            read_dir_options.reverse = !read_dir_options.reverse;
                        } else {
                            read_dir_options.sorting = FileSorting::Extension;
                        }
                        send_event(KekEvent::RefreshDirList);
                    }
                });
                header.col(|ui| {
                    if ui.add(clickable_label("Size")).clicked() {
                        if read_dir_options.sorting == FileSorting::Size {
                            read_dir_options.reverse = !read_dir_options.reverse;
                        } else {
                            read_dir_options.sorting = FileSorting::Size;
                        }
                        send_event(KekEvent::RefreshDirList);
                    }
                });
            })
            .body(|body| {
                body.rows(
                    text_height,
                    items.len(),
                    |index, mut row| {
                        if let Some(i) = items.get(index) {
                            row.col(|ui| {
                                if i.is_dir {
                                    ui.add(egui::Image::new(textures.folder.id(), icon_size));
                                } else {
                                    ui.add(egui::Image::new(textures.file.id(), icon_size));
                                }

                                if ui
                                    .add(clickable_label(&i.filename))
                                    .context_menu(|ui| file_right_click(ui, i))
                                    .double_clicked()
                                {
                                    send_event(KekEvent::Navigate(i.path.clone()));
                                }
                            });
                            row.col(|ui| {
                                ui.label(&i.extension);
                            });
                            row.col(|ui| {
                                ui.label(bytes_to_human_readable(i.size_bytes));
                            });
                        }
                    },
                );
            });
    }
}

fn bytes_to_human_readable(bytes: u64) -> String {
    let bytes = bytes as f64;
    match bytes {
        x if x < 1024.0 => format!("{} bytes", bytes),
        x if x < 1_048_576.0 => format!("{:.1} KiB", bytes / 1024.00),
        x if x < 1_073_741_824.0 => format!("{:.2} MiB", bytes / 1024.00 / 1024.00),
        x if x < 1_099_511_627_776.0 => format!("{:.2} GiB", bytes / 1024.00 / 1024.00 / 1024.00),
        x if x < 1_125_899_906_842_624.0 => {
            format!("{:.2} TiB", bytes / 1024.00 / 1024.00 / 1024.00 / 1024.00)
        }
        _ => String::from("Very large"),
    }
}
