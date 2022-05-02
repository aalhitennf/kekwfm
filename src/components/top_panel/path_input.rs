use eframe::egui::{self};

use crate::app::KekwFM;

// Since we cant just tell layout to fill available we have to set the desired width with manual margin
pub fn build(ui: &mut egui::Ui, app: &mut KekwFM, right_margin: f32) {
    let input = egui::TextEdit::singleline(&mut app.input_value)
        .margin(egui::vec2(10.0, 8.0))
        .desired_width(ui.available_width() - right_margin)
        .lock_focus(true)
        .cursor_at_end(true);

    let response = ui.add(input);

    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
        response.request_focus();
        app.try_navigate(None);
    }
}
