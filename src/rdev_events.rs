// use eframe::egui;
use rdev::{listen, Button, EventType};

use crate::eevertti::{send_event, KekEvent, MouseButton};

pub fn spawn_mouse_event_listener(ctx: eframe::egui::Context) {
    std::thread::spawn(move || listen_events(ctx));
}

fn listen_events(ctx: eframe::egui::Context) {
    if let Err(error) = listen(move |event| {
        match event.event_type {
            EventType::ButtonPress(Button::Unknown(8)) => {
                if ctx.is_pointer_over_area() {
                    send_event(KekEvent::ButtonPress(MouseButton::Back));
                    ctx.request_repaint();
                }
            }
            EventType::ButtonPress(Button::Unknown(9)) => {
                if ctx.is_pointer_over_area() {
                    send_event(KekEvent::ButtonPress(MouseButton::Forward));
                    ctx.request_repaint();
                }
            }
            _ => (),
        }
    }) {
        println!("Error: {:?}", error)
    }
}
