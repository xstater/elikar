extern crate sdl2_sys;

use sdl2_sys::SDL_MouseMotionEvent;
use crate::mouse::ButtonState;
use crate::window::WindowId;

#[derive(Debug,Clone,Copy)]
pub struct EventInfo {
    pub timestamp : u32,
    pub window_id : WindowId,
    pub which : u32,
    pub state : ButtonState,
    pub position : (i32,i32),
    pub relative : (i32,i32)
}

impl From<SDL_MouseMotionEvent> for EventInfo {
    fn from(sdl_motion : SDL_MouseMotionEvent) -> Self {
        EventInfo {
            timestamp: sdl_motion.timestamp,
            window_id: WindowId::from_u32(sdl_motion.windowID),
            which: sdl_motion.which,
            state: ButtonState::new(sdl_motion.state),
            position: (sdl_motion.x, sdl_motion.y),
            relative: (sdl_motion.xrel, sdl_motion.yrel),
        }
    }
}