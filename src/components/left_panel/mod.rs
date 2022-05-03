use eframe::{egui::{self, Button, Frame, Sense, Ui, Response, style::Margin}, emath::Vec2, epaint::TextureId};
use kekwlib::locations::Locations;

use crate::app::KekwFM;

const ICON_SIZE: Vec2 = egui::vec2(18.0, 18.0);
const ITEM_SIZE: [f32; 2] = [150.0, 20.0];

const LIST_SPACING_LABEL: f32 = 5.0;
const LIST_SPACING_SECTION: f32 = 15.0;
pub struct LeftPanel;

fn create_location_item(ui: &mut Ui, text: &str, texture: TextureId) -> Response {
    let button = Button::image_and_text(texture, ICON_SIZE, text)
        .frame(false)
        .sense(Sense::click());

    ui.add_sized(ITEM_SIZE, button)
}

pub fn build(ctx: &egui::Context, app: &mut KekwFM, locations: Locations) {

    let frame = Frame {
        inner_margin: Margin::symmetric(10.0, 5.0),
        fill: ctx.style().visuals.window_fill(),
        ..Frame::default()
    };

    egui::SidePanel::left("left_panel")
        .frame(frame)
        .resizable(true)
        .show(ctx, |ui| {

            ui.label("Locations");
            ui.add_space(LIST_SPACING_LABEL);

            if create_location_item(ui, &locations.home.text, app.textures.home.id()).clicked() {
                app.try_navigate(Some(locations.home.path.clone()));
            }

            ui.add_space(LIST_SPACING_SECTION);


            ui.label("Favourites");
            ui.add_space(LIST_SPACING_LABEL);

            for fav in locations.favourites {
                if create_location_item(ui, &fav.text, app.textures.star.id()).clicked() {
                    app.try_navigate(Some(fav.path.clone()));
                }
            }

            ui.add_space(LIST_SPACING_SECTION);
            

            ui.label("Devices");
            ui.add_space(LIST_SPACING_LABEL);

            for dev in locations.devices {
                if create_location_item(ui, &dev.info.device_name, app.textures.hard_drive.id()).clicked() {
                    app.try_navigate(Some(dev.info.mount_point.clone()));
                }
            }
        });
}
