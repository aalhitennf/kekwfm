use rdev::{listen, Button, EventType};

use crate::eevertti::{send_event, KekEvent};

pub fn spawn_mouse_event_listener(ctx: eframe::egui::Context) {
    std::thread::spawn(move || listen_events(ctx));
}

// Since egui doesnt support mouse back/forward button events
// listen for them with rdev
fn listen_events(ctx: eframe::egui::Context) {
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::ButtonPress(Button::Unknown(8)) => {
            if ctx.is_pointer_over_area() {
                send_event(KekEvent::NavigateBack);
                ctx.request_repaint();
            }
        }
        EventType::ButtonPress(Button::Unknown(9)) => {
            if ctx.is_pointer_over_area() {
                send_event(KekEvent::NavigateForward);
                ctx.request_repaint();
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error);
    }
}
