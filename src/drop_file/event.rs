extern crate sdl2_sys;

use std::path::PathBuf;
use sdl2_sys::SDL_DropEvent;

#[derive(Debug, Clone)]
pub struct Info{
    pub timestamp : u32,
    pub window_id : u32,
    pub paths : Vec<PathBuf>,
}

impl Info{
    pub(in crate) fn add_path(&mut self, path : PathBuf){
        self.paths.push(path)
    }
}

impl From<SDL_DropEvent> for Info{
    fn from(event : SDL_DropEvent) -> Self {
        Info{
            timestamp: event.timestamp,
            window_id: event.windowID,
            paths : Vec::new(),
        }
    }
}
