use std::ffi::CStr;
use std::ops::Range;
use crate::window::WindowId;
use sdl2_sys::*;

pub struct TextEditing {
    pub timestamp : u32,
    pub window_id : WindowId,
    pub text : String,
    pub range : Range<i32>
}

impl From<SDL_TextEditingEvent> for TextEditing {
    fn from(event: SDL_TextEditingEvent) -> Self {
        TextEditing {
            timestamp: event.timestamp,
            window_id: WindowId::from_u32(event.windowID),
            text: unsafe {
                CStr::from_ptr(event.text.as_ptr())
            }.to_str().unwrap().to_owned(),
            range: event.start .. (event.start + event.length)
        }
    }
}