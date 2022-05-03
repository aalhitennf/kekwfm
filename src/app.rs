use kekwlib::{dirutils::{path_is_dir, read_directory_listing, DirectoryListing, ReadDirOptions}, locations::Locations};

use crate::{
    components::{self, top_panel::TopPanel, list_view::ListView},
    textures::TextureLoader,
};

use eframe::egui;

// #[derive(Default)]
pub struct KekwFM {
    pub current_path: String,
    pub input_value: String,
    pub input_error: Option<String>,
    pub directory_listing: DirectoryListing,
    pub textures: TextureLoader,
    pub settings_visible: bool,
    pub read_dir_options: ReadDirOptions,
    pub locations: Locations,
}

impl KekwFM {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.2);


        let textures = TextureLoader::new("feather", &cc.egui_ctx);
        // let loaded = cc.egui_ctx.tex_manager().read().num_allocated();

        // TODO LOAD TEXTURES HERE AND CHANGE TEXTURES STRUCT TO HOLD ONLY IDS OF THE LOADED THINS

        let dirs = directories::UserDirs::new().unwrap();


        let default_path = dirs.home_dir().to_str().unwrap().to_string();

        let read_dir_options = ReadDirOptions::default();


        let directory_listing = read_directory_listing(&default_path, &read_dir_options)
            .map_or(DirectoryListing::default(), |result| result);

        let start = std::time::Instant::now();


        let locations = Locations::default(); // TODO This is slow as fuck

        println!("loaded locations in: {} ms", start.elapsed().as_millis());


        Self {
            current_path: default_path.clone(),
            input_value: default_path,
            input_error: None,
            directory_listing,
            textures,
            settings_visible: false,
            read_dir_options,
            locations,
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

        components::left_panel::build(ctx, self, self.locations.clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO This clone is bad but it'll do for now

            // let items = self.directory_listing.items.clone();


            ListView::build(ui, self, self.directory_listing.items.clone());
        });
    }
}

// egui::ScrollArea::vertical()
//     .auto_shrink([false, true])
//     .show_rows(ui, row_height, num_rows, |ui, row_range| {
//         for row in row_range {
//             let item = self.directory_listing.items.get(row);

//             if let Some(item) = item {
//                 if ui
//                     .add(
//                         egui::Label::new(WidgetText::from(item))
//                             .sense(egui::Sense::click()),
//                     )
//                     .double_clicked()
//                 {
//                     let p = item.path.clone();
//                     self.try_navigate(Some(p));
//                 };
//             }
//         }
//     });