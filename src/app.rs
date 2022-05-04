use std::path::Path;

use crossbeam_channel::Receiver;
use directories::UserDirs;
use kekwlib::{
    dirutils::{read_directory_listing, DirectoryListing, ReadDirOptions},
    fileutils,
    history::History,
    observer::FsObserver,
};

use crate::{
    components::{LeftPanel, ListView, TopPanel},
    eevertti::{set_eevertti, KekEvent, MouseButton},
    rdev_events,
    textures::TextureLoader,
};

use eframe::egui;

pub struct KekwFM {
    pub dirs: UserDirs,
    history: History,
    pub input_error: Option<String>,
    pub directory_listing: DirectoryListing,
    pub textures: TextureLoader,
    // UI elements
    pub left_panel: LeftPanel,
    pub top_panel: TopPanel,
    pub list_view: ListView,
    pub read_dir_options: ReadDirOptions,
    pub observer: FsObserver,
    pub event_receiver: Receiver<KekEvent>,
}

impl KekwFM {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.2);

        let textures = TextureLoader::new("feather", &cc.egui_ctx);

        let dirs = directories::UserDirs::new().unwrap();

        let default_path = dirs.home_dir().to_str().unwrap().to_string();

        let history = History::new(&default_path);

        let read_dir_options = ReadDirOptions::default();

        let directory_listing = read_directory_listing(&default_path, &read_dir_options)
            .map_or(DirectoryListing::default(), |result| result);

        let observer = FsObserver::new(&default_path, cc.egui_ctx.clone());

        let (tx, rx) = crossbeam_channel::unbounded();

        set_eevertti(tx);

        // Spawn mouse button listener
        rdev_events::spawn_mouse_event_listener(cc.egui_ctx.clone());

        // UI Elements
        let left_panel = LeftPanel::new();
        let top_panel = TopPanel::new(&default_path);
        let list_view = ListView::default();

        Self {
            dirs,
            history,
            input_error: None,
            directory_listing,
            textures,
            left_panel,
            top_panel,
            list_view,
            read_dir_options,
            observer,
            event_receiver: rx,
        }
    }

    // Take optional path as argument, if none passed, try navigate to input value
    fn try_navigate<P: AsRef<Path> + ToString + Copy>(&mut self, path: P) {
        if !path.as_ref().is_dir() {
            return;
        }

        if path.to_string() == self.observer.path {
            return;
        }

        self.refresh_dir_listing(path);
        self.observer.change_path(path);
        self.top_panel.input_value = path.to_string();
        self.history.add(&path.to_string());
    }

    fn try_navigate_parent(&mut self) {
        if let Some(parent) = &self.directory_listing.parent {
            let p = parent.clone();
            self.try_navigate(&p);
        }
    }

    fn try_navigate_back(&mut self) {
        if let Some(previous) = self.history.get_previous() {
            self.try_navigate(&previous);
        }
    }

    fn try_navigate_forward(&mut self) {
        if let Some(next) = self.history.get_next() {
            self.try_navigate(&next);
        }
    }

    fn refresh_dir_listing<P: AsRef<Path> + Copy>(&mut self, path: P) {
        match read_directory_listing(path, &self.read_dir_options) {
            Ok(result) => {
                self.directory_listing = result;
                // Reset various ui items
                self.list_view.all_selected = false;
            }
            Err(e) => {
                println!("{}", e);
                self.input_error = Some(e.to_string());
            }
        }
    }

    fn refresh_current_dir_listing(&mut self) {
        match read_directory_listing(&self.observer.path, &self.read_dir_options) {
            Ok(result) => {
                self.directory_listing = result;
                // Reset various ui items
                self.list_view.all_selected = false;
            }
            Err(e) => {
                println!("{}", e);
                self.input_error = Some(e.to_string());
            }
        }
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        self.top_panel
            .show(ctx, &self.textures, &mut self.read_dir_options);
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        self.left_panel.show(ctx, &self.textures);
    }
}

impl eframe::App for KekwFM {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.observer.receiver.try_recv().is_ok() {
            self.refresh_current_dir_listing();
        }

        if let Ok(event) = self.event_receiver.try_recv() {
            match event {
                KekEvent::Print(text) => println!("Eeventti: {}", text),
                KekEvent::Navigate(path) => self.try_navigate(&path),
                KekEvent::NavigateParent => self.try_navigate_parent(),
                KekEvent::NavigateBack => self.try_navigate_back(),
                KekEvent::NavigateForward => self.try_navigate_forward(),
                KekEvent::RefreshDirList => self.refresh_current_dir_listing(),
                KekEvent::XdgOpenFile(path) => fileutils::xdg_open_file(&path),
                KekEvent::ButtonPress(MouseButton::Back) => self.try_navigate_back(),
                KekEvent::ButtonPress(MouseButton::Forward) => self.try_navigate_forward(),
                _ => println!("Unimplemented event"),
            }
        }

        self.top_panel(ctx);

        self.left_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.list_view.show(
                ui,
                &mut self.directory_listing.items,
                &self.textures,
                &mut self.read_dir_options,
            );
        });
    }
}
