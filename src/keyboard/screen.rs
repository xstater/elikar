extern crate sdl2_sys;

use crate::window::Window;
use sdl2_sys::*;

pub struct ScreenKeyboard;

impl ScreenKeyboard {
    pub fn has_support(&self) -> bool {
        unsafe { SDL_HasScreenKeyboardSupport() == SDL_bool::SDL_TRUE }
    }

    pub fn is_shown(&self, window: &Window) -> bool {
        let window_ptr = unsafe { window.window_ptr() };
        return unsafe { SDL_IsScreenKeyboardShown(window_ptr) == SDL_bool::SDL_TRUE };
    }
}
