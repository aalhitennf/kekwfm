use eframe::{
    egui::{self, style::Margin, Button, Frame, Response, Sense, Ui},
    emath::Vec2,
    epaint::{TextureId, Color32},
};
use kekwlib::locations::Locations;

use crate::{
    eevertti::{send_event, KekEvent},
    textures::Textures,
};

const ICON_SIZE: Vec2 = egui::vec2(18.0, 18.0);
const ITEM_SIZE: [f32; 2] = [130.0, 20.0];

const LIST_SPACING_LABEL: f32 = 5.0;
const LIST_SPACING_SECTION: f32 = 15.0;

fn create_location_item(ui: &mut Ui, text: &str, texture: TextureId) -> Response {
    let button = Button::image_and_text(texture, ICON_SIZE, text)
        .frame(false)
        .sense(Sense::click());

    ui.add_sized(ITEM_SIZE, button)
}

pub struct LeftPanel {
    locations: Locations,
    frame: Frame,
}

impl LeftPanel {
    pub fn new(fill: Color32) -> Self {
        let locations = Locations::default();
        let frame = Frame {
            inner_margin: Margin::symmetric(10.0, 5.0),
            fill,
            ..Frame::default()
        };
        LeftPanel { locations, frame }
    }

    pub fn show(&mut self, ctx: &egui::Context, textures: &Textures) {
        egui::SidePanel::left("left_panel")
            .frame(self.frame)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Locations");
                ui.add_space(LIST_SPACING_LABEL);

                if create_location_item(ui, &self.locations.home.text, textures.home.id()).clicked()
                {
                    send_event(KekEvent::Navigate(self.locations.home.path.clone()));
                }

                ui.add_space(LIST_SPACING_SECTION);

                ui.label("Favourites");
                ui.add_space(LIST_SPACING_LABEL);

                for fav in self.locations.favourites.iter() {
                    if create_location_item(ui, &fav.text, textures.star.id()).clicked() {
                        send_event(KekEvent::Navigate(fav.path.clone()));
                    }
                }

                ui.add_space(LIST_SPACING_SECTION);

                ui.label("Devices");
                ui.add_space(LIST_SPACING_LABEL);

                for dev in self.locations.devices.iter() {
                    if create_location_item(ui, &dev.info.device_name, textures.hard_drive.id())
                        .clicked()
                    {
                        send_event(KekEvent::Navigate(dev.info.mount_point.clone()));
                    }
                }
            });
    }
}
