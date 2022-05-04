use eframe::{
    egui::{self, Button, Layout, PointerButton, Sense, TextStyle},
    emath::Vec2,
    epaint::TextureId,
};
use egui_extras::{Size, TableBuilder};

use kekwlib::dirutils::{DirectoryListing, DirectoryListingItem, FileSorting, ReadDirOptions};

use crate::{
    components::context_menu::file_right_click,
    eevertti::{send_event, KekEvent},
    textures::TextureLoader,
};

const HEADER_SIZE: f32 = 20.0;
const ICON_SIZE: Vec2 = Vec2 { x: 18.0, y: 18.0 };

fn clickable_label(text: &str) -> egui::Label {
    egui::Label::new(text).sense(Sense::click()).wrap(false)
}

pub trait DirectoryView {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        read_dir_options: &mut ReadDirOptions,
        textures: &TextureLoader,
        items: &DirectoryListing,
    );
}

#[derive(Default)]
pub struct ListView {
    pub all_selected: bool,
}

impl ListView {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        items: &mut [DirectoryListingItem],
        textures: &TextureLoader,
        read_dir_options: &mut ReadDirOptions,
    ) {
        let text_height = TextStyle::Body.resolve(ui.style()).size + 10.0;

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(450.0).at_least(200.0))
            .column(Size::relative(0.1))
            .column(Size::remainder())
            .resizable(true)
            .header(HEADER_SIZE, |mut header| {
                header.col(|ui| {
                    if ui
                        .add(egui::Checkbox::new(&mut self.all_selected, ""))
                        .changed()
                    {
                        for item in items.iter_mut() {
                            item.selected = self.all_selected;
                        }
                    }

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
                body.rows(text_height, items.len(), |index, mut row| {
                    if let Some(mut i) = items.get_mut(index) {
                        let icon = if i.is_dir {
                            textures.folder.id()
                        } else {
                            textures.file.id()
                        };
                        row.col(|ui| {
                            row_file_item(ui, &mut i, icon);
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

fn row_file_item(ui: &mut egui::Ui, item: &mut DirectoryListingItem, icon: TextureId) {
    let r = ui.with_layout(Layout::left_to_right().with_main_justify(false), |ui| {
        ui.add(egui::Checkbox::new(&mut item.selected, ""));

        ui.with_layout(Layout::left_to_right().with_main_justify(true), |ui| {
            ui.add(Button::image_and_text(icon, ICON_SIZE, &item.filename).frame(false))
                .context_menu(|ui| file_right_click(ui, item))
        })
    });

    if ui.input().modifiers.shift || ui.input().modifiers.ctrl {
        if r.inner.inner.clicked_by(PointerButton::Primary) {
            item.selected = !item.selected;
        }
    } else {
        if r.inner.inner.double_clicked_by(PointerButton::Primary) {
            if item.is_dir {
                send_event(KekEvent::Navigate(item.path.clone()));
            } else if item.is_file {
                send_event(KekEvent::XdgOpenFile(item.path.clone()));
            }
        }
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
