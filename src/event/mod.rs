use xrsignal::Signal;

pub mod mouse_event;

pub struct EventHandlers{
    pub mouse_button_down : Signal<(i32,i32),()>,
}

impl EventHandlers{
    pub fn new() -> EventHandlers {
        EventHandlers{
            mouse_button_down: Signal::new(),
        }
    }
}