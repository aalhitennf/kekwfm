use eframe::{
    egui::{self, style::Margin, Align, Frame, ImageButton, Layout, Response, Vec2},
    epaint::TextureId,
};
use kekwlib::dirutils::{FileSorting, ReadDirOptions};

use crate::{
    eevertti::{send_event, KekEvent},
    textures::TextureLoader,
};

mod path_input;

const MARGIN: f32 = 5.0;
const PANEL_HEIGHT: f32 = 40.0;
const BUTTON_WIDTH: f32 = 30.0;
const BUTTON_HEIGHT: f32 = PANEL_HEIGHT - (MARGIN + MARGIN);
const ICON_SIZE: Vec2 = Vec2 {
    x: BUTTON_WIDTH - 8.0,
    y: BUTTON_HEIGHT - 8.0,
};

/// Give amount of items in f32 so theres no need for conversions
fn calculate_input_margin(items: f32, spacing: f32) -> f32 {
    let b = (items + 0.5) * BUTTON_WIDTH;
    let s = spacing * items;
    b + s + MARGIN
}

fn create_button(ui: &mut egui::Ui, texture_id: TextureId) -> Response {
    ui.add_sized(
        [BUTTON_WIDTH, BUTTON_HEIGHT],
        ImageButton::new(texture_id, ICON_SIZE).frame(false),
    )
}

// #[derive(Default)]
pub struct TopPanel {
    settings_visible: bool,
    pub input_value: String,
}

impl TopPanel {
    pub fn new(input_value: &str) -> Self {
        TopPanel {
            settings_visible: false,
            input_value: input_value.to_string(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        textures: &TextureLoader,
        read_dir_options: &mut ReadDirOptions,
    ) {
        let frame = Frame {
            inner_margin: Margin::same(MARGIN),
            fill: ctx.style().visuals.window_fill(),
            ..Frame::default()
        };

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .max_height(PANEL_HEIGHT)
            .frame(frame)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        if create_button(ui, textures.arrow_left.id()).clicked() {
                            send_event(KekEvent::NavigateParent);
                        }

                        if create_button(ui, textures.arrow_right.id()).clicked() {
                            send_event(KekEvent::Print(String::from("Unimplemented")));
                        }

                        path_input::build(
                            ui,
                            &mut self.input_value,
                            calculate_input_margin(1.0, ui.spacing().item_spacing.x),
                        );

                        if create_button(ui, textures.settings.id()).clicked() {
                            self.settings_visible = !self.settings_visible;
                        }
                    });

                    if self.settings_visible {
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            if ui
                                .add(egui::Checkbox::new(
                                    &mut read_dir_options.include_hidden,
                                    "Show hidden files",
                                ))
                                .changed()
                            {
                                send_event(KekEvent::RefreshDirList);
                            }

                            if ui
                                .add(egui::Checkbox::new(
                                    &mut read_dir_options.folders_first,
                                    "Folders first",
                                ))
                                .changed()
                            {
                                send_event(KekEvent::RefreshDirList);
                            }

                            if ui
                                .add(egui::Checkbox::new(
                                    &mut read_dir_options.reverse,
                                    "Reverse order",
                                ))
                                .changed()
                            {
                                send_event(KekEvent::RefreshDirList);
                            }

                            egui::ComboBox::from_label("Sorting")
                                .selected_text(format!("{:?}", read_dir_options.sorting))
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_value(
                                            &mut read_dir_options.sorting,
                                            FileSorting::Alphabetical,
                                            "Alphabetical",
                                        )
                                        .changed()
                                    {
                                        send_event(KekEvent::RefreshDirList);
                                    };

                                    if ui
                                        .selectable_value(
                                            &mut read_dir_options.sorting,
                                            FileSorting::Size,
                                            "Size",
                                        )
                                        .changed()
                                    {
                                        send_event(KekEvent::RefreshDirList);
                                    }

                                    if ui
                                        .selectable_value(
                                            &mut read_dir_options.sorting,
                                            FileSorting::Extension,
                                            "Type",
                                        )
                                        .changed()
                                    {
                                        send_event(KekEvent::RefreshDirList);
                                    }

                                    if ui
                                        .selectable_value(
                                            &mut read_dir_options.sorting,
                                            FileSorting::None,
                                            "None",
                                        )
                                        .changed()
                                    {
                                        send_event(KekEvent::RefreshDirList);
                                    }
                                });
                        });
                    }

                    ui.separator();
                });
            });
    }
}
