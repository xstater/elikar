use crate::window::WindowId;
use sdl2_sys::*;
use std::ffi::CStr;

pub struct TextInput {
    pub timestamp: u32,
    pub window_id: WindowId,
    pub text: String,
}

impl From<SDL_TextInputEvent> for TextInput {
    fn from(event: SDL_TextInputEvent) -> Self {
        TextInput {
            timestamp: event.timestamp,
            window_id: WindowId::from_u32(event.windowID),
            text: unsafe { CStr::from_ptr(event.text.as_ptr()) }
                .to_str()
                .unwrap()
                .to_owned(),
        }
    }
}
