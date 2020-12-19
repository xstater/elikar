extern crate sdl2_sys;

use sdl2_sys::SDL_MouseMotionEvent;
use crate::mouse::ButtonState;

#[derive(Debug,Clone,Copy)]
pub struct Info{
    pub timestamp : u32,
    pub window_id : u32,
    pub which : u32,
    pub state : ButtonState,
    pub position : (i32,i32),
    pub relative : (i32,i32)
}

impl From<SDL_MouseMotionEvent> for Info{
    fn from(sdl_motion : SDL_MouseMotionEvent) -> Self {
        Info{
            timestamp: sdl_motion.timestamp,
            window_id: sdl_motion.windowID,
            which: sdl_motion.which,
            state: ButtonState::new(sdl_motion.state),
            position: (sdl_motion.x, sdl_motion.y),
            relative: (sdl_motion.xrel, sdl_motion.yrel),
        }
    }
}