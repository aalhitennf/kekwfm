use eframe::{egui::{self, style::Margin, Align, Frame, Layout, Vec2}, epaint::Color32};
use kekwlib::dirutils::FileSorting;

use crate::app::KekwFM;

mod path_input;

const MARGIN: f32 = 5.0;
const PANEL_HEIGHT: f32 = 40.0;
const BUTTON_WIDTH: f32 = 30.0;
const BUTTON_HEIGHT: f32 = PANEL_HEIGHT - (MARGIN + MARGIN);
const ICON_SIZE: Vec2 = Vec2 { x: BUTTON_WIDTH - 8.0, y: BUTTON_HEIGHT - 8.0 };

/// Give amount of items in f32 so theres no need for conversions
fn calculate_input_margin(items: f32, spacing: f32) -> f32 {
    let b = (items + 0.5) * BUTTON_WIDTH;
    let s = spacing * items;
    b + s + MARGIN
}
pub struct TopPanel;

impl TopPanel {
    pub fn build(ctx: &egui::Context, app: &mut KekwFM) {

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
                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, BUTTON_HEIGHT],
                                egui::ImageButton::new(app.textures.arrow_left.id(), ICON_SIZE).frame(false),
                            )
                            .clicked()
                        {
                            app.try_navigate_parent()
                        }

                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, BUTTON_HEIGHT],
                                egui::ImageButton::new(app.textures.arrow_right.id(), ICON_SIZE).frame(false),
                            )
                            .clicked()
                        {
                            app.unimplemented();
                        }

                        path_input::build(
                            ui,
                            app,
                            calculate_input_margin(1.0, ui.spacing().item_spacing.x),
                        );

                        if ui
                            .add_sized(
                                [BUTTON_WIDTH, BUTTON_HEIGHT],
                                egui::ImageButton::new(app.textures.settings.id(), ICON_SIZE).frame(false),
                            )
                            .clicked()
                        {
                            app.settings_visible = !app.settings_visible;
                        }
                    });

                    if app.settings_visible {
                        ui.with_layout(Layout::right_to_left(), |ui| {
                            if ui
                                .add(egui::Checkbox::new(
                                    &mut app.read_dir_options.include_hidden,
                                    "Show hidden files",
                                ))
                                .changed()
                            {
                                app.refresh_current_dir_listing();
                            }

                            if ui
                                .add(egui::Checkbox::new(
                                    &mut app.read_dir_options.folders_first,
                                    "Folders first",
                                ))
                                .changed()
                            {
                                app.refresh_current_dir_listing();
                            }

                            if ui
                                .add(egui::Checkbox::new(
                                    &mut app.read_dir_options.reverse,
                                    "Reverse order",
                                ))
                                .changed()
                            {
                                app.refresh_current_dir_listing();
                            }

                            egui::ComboBox::from_label("Sorting")
                                .selected_text(format!("{:?}", app.read_dir_options.sorting))
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_value(
                                            &mut app.read_dir_options.sorting,
                                            FileSorting::Alphabetical,
                                            "Alphabetical",
                                        )
                                        .changed()
                                    {
                                        app.refresh_current_dir_listing();
                                    };
                                    if ui
                                        .selectable_value(
                                            &mut app.read_dir_options.sorting,
                                            FileSorting::Size,
                                            "Size",
                                        )
                                        .changed()
                                    {
                                        app.refresh_current_dir_listing();
                                    }
                                    if ui
                                        .selectable_value(
                                            &mut app.read_dir_options.sorting,
                                            FileSorting::Extension,
                                            "Type",
                                        )
                                        .changed()
                                    {
                                        app.refresh_current_dir_listing();
                                    }
                                    if ui
                                        .selectable_value(
                                            &mut app.read_dir_options.sorting,
                                            FileSorting::None,
                                            "None",
                                        )
                                        .changed()
                                    {
                                        app.refresh_current_dir_listing();
                                    }
                                });
                        });
                    }

                    ui.separator();
                });
            });
    }
}
