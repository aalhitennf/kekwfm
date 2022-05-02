use kekwlib::dirutils::{path_is_dir, read_directory_listing, DirectoryListing, ReadDirOptions};

use crate::{
    components::{self, top_panel::TopPanel},
    textures::Textures,
};

use eframe::egui::{self, WidgetText};

#[derive(Default)]
pub struct KekwFM {
    pub current_path: String,
    pub input_value: String,
    pub input_error: Option<String>,
    pub directory_listing: DirectoryListing,
    pub textures: Textures,
    pub settings_visible: bool,
    pub read_dir_options: ReadDirOptions,
}

impl KekwFM {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.2);

        let dirs = directories::UserDirs::new().unwrap();
        let default_path = dirs.home_dir().to_str().unwrap().to_string();

        let read_dir_options = ReadDirOptions::default();

        let directory_listing = read_directory_listing(&default_path, &read_dir_options)
            .map_or(DirectoryListing::default(), |result| result);
        Self {
            current_path: default_path.clone(),
            input_value: default_path,
            input_error: None,
            directory_listing,
            textures: Textures::new("feather"),
            settings_visible: false,
            read_dir_options,
        }
    }

    // Take optional path as argument, if none passed, try navigate to input value
    pub fn try_navigate(&mut self, path: Option<String>) {
        let path = if let Some(p) = path {
            p
        } else {
            self.input_value.clone()
        };
        let path = if path.len() > 1 {
            path.trim_end_matches('/').to_string()
        } else {
            path
        };

        if !path_is_dir(&path) {
            self.input_error = Some(format!("Path is not directory: {}", path));
            return;
        }

        self.refresh_dir_listing(&path);
        self.current_path = path.to_string();
        self.input_value = path.to_string();
    }

    pub fn try_navigate_parent(&mut self) {
        if let Some(parent) = &self.directory_listing.parent {
            let p = parent.clone();
            self.try_navigate(Some(p));
        }
    }

    pub fn try_navigate_forward(&self) {
        println!("Unimplementd");
    }

    pub fn refresh_dir_listing(&mut self, path: &str) {
        match read_directory_listing(path, &self.read_dir_options) {
            Ok(result) => {
                self.directory_listing = result;
            }
            Err(e) => {
                println!("{}", e.to_string());
                self.input_error = Some(e.to_string());
            }
        }
    }

    pub fn refresh_current_dir_listing(&mut self) {
        match read_directory_listing(&self.current_path, &self.read_dir_options) {
            Ok(result) => {
                self.directory_listing = result;
            }
            Err(e) => {
                println!("{}", e.to_string());
                self.input_error = Some(e.to_string());
            }
        }
    }

    pub fn unimplemented(&self) {
        println!("Unimplementd");
    }
}

impl eframe::App for KekwFM {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopPanel::build(ctx, self);

        components::left_panel::build(ctx, self);

        egui::CentralPanel::default().show(ctx, |ui| {
            let font_id = egui::TextStyle::Body.resolve(ui.style());
            let row_height = ui.fonts().row_height(&font_id);
            let num_rows = self.directory_listing.items.len();

            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for row in row_range {
                        let item = self.directory_listing.items.get(row);

                        if let Some(item) = item {
                            if ui
                                .add(
                                    egui::Label::new(WidgetText::from(item))
                                        .sense(egui::Sense::click()),
                                )
                                .double_clicked()
                            {
                                let p = item.path.clone();
                                self.try_navigate(Some(p));
                            };
                        }
                    }
                });
        });
    }
}
