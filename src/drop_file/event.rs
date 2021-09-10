extern crate sdl2_sys;

use std::path::PathBuf;
use sdl2_sys::SDL_DropEvent;
use crate::window::WindowId;

#[derive(Debug, Clone)]
pub struct EventInfo {
    pub timestamp : u32,
    pub window_id : WindowId,
    pub paths : Vec<PathBuf>,
}

impl EventInfo {
    pub(in crate) fn add_path(&mut self, path : PathBuf){
        self.paths.push(path)
    }
}

impl From<SDL_DropEvent> for EventInfo {
    fn from(event : SDL_DropEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id: WindowId::from_u32(event.windowID),
            paths : Vec::new(),
        }
    }
}
