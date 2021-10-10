extern crate sdl2_sys;

use crate::keyboard::{Code, Mod};
use crate::window::WindowId;
use sdl2_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum State {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: WindowId,
    pub state: State,
    pub is_repeat: bool,
    pub code: Code,
    pub mod_state: Mod,
}

impl From<SDL_KeyboardEvent> for EventInfo {
    fn from(event: SDL_KeyboardEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id: WindowId::from_u32(event.windowID),
            state: if event.state == SDL_PRESSED as u8 {
                State::Pressed
            } else {
                State::Released
            },
            is_repeat: event.repeat != 0,
            code: event.keysym.scancode.into(),
            mod_state: Mod::new(event.keysym.mod_ as u32),
        }
    }
}
