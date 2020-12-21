extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::Window;

pub fn has_support() -> bool{
    unsafe { SDL_HasScreenKeyboardSupport() == SDL_bool::SDL_TRUE }
}

pub fn is_shown(window : &Window) -> bool{
    let window_cloned = window.clone();
    let (manager,window_ptr ) = unsafe{window_cloned.split()};
    if let Some(ptr) = manager.upgrade() {
        if let Ok(_guard) = ptr.read() {
            return unsafe{
                SDL_IsScreenKeyboardShown(window_ptr) == SDL_bool::SDL_TRUE
            };
        }
    }
    false
}