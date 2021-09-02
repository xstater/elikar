extern crate sdl2_sys;

use sdl2_sys::*;

#[derive(Debug,Clone,Copy)]
pub struct EventInfo {
    pub timestamp : u32,
    pub window_id : u32,
    pub which : u32,
    pub scrolled : (i32,i32)
}

impl From<SDL_MouseWheelEvent> for EventInfo {
    fn from(sdl_wheel : SDL_MouseWheelEvent) -> Self {
        EventInfo {
            timestamp: sdl_wheel.timestamp,
            window_id: sdl_wheel.windowID,
            which: sdl_wheel.which,
            scrolled: if sdl_wheel.direction == SDL_MouseWheelDirection::SDL_MOUSEWHEEL_NORMAL as u32 {
                (sdl_wheel.x,sdl_wheel.y)
            }else{
                (-sdl_wheel.x,-sdl_wheel.y)
            }
        }
    }
}