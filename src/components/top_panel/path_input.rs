use eframe::egui;

use crate::eevertti::{send_event, KekEvent};

// Since we cant just tell layout to fill available we have to set the desired width with manual margin
#[allow(clippy::ptr_arg)]
pub fn build(ui: &mut egui::Ui, value: &mut String, right_margin: f32) {
    let input = egui::TextEdit::singleline(value)
        .margin(egui::vec2(10.0, 8.0))
        .desired_width(ui.available_width() - right_margin)
        .lock_focus(true)
        .cursor_at_end(true);

    let response = ui.add(input);

    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
        response.request_focus();
        send_event(KekEvent::Navigate(value.clone()));
    }

    if response.has_focus() && ui.input().key_pressed(egui::Key::Tab) {
        println!("Tab pressed");
    }
}
