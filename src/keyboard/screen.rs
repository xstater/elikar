extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::Window;

pub fn has_support() -> bool{
    unsafe { SDL_HasScreenKeyboardSupport() == SDL_bool::SDL_TRUE }
}

pub fn is_shown(window : &Window) -> bool{
    let window_ptr = unsafe { window.window_ptr() };
    return unsafe {
        SDL_IsScreenKeyboardShown(window_ptr) == SDL_bool::SDL_TRUE
    }
}