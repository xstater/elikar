use std::marker::PhantomData;
use sdl2_sys::{SDL_HasScreenKeyboardSupport, SDL_IsScreenKeyboardShown, SDL_IsTextInputActive, SDL_Rect, SDL_SetTextInputRect, SDL_StartTextInput, SDL_StopTextInput, SDL_bool};
use xecs::resource::Resource;
use crate::window::Window;

pub mod events;

pub struct IME {
    // To avoid be constructed by user
    _marker : PhantomData<()>
}

impl Resource for IME {}

impl IME {
    pub(in crate) fn new() -> Self {
        IME {
            _marker: Default::default(),
        }
    }

    pub fn start(&mut self) {
        unsafe {
            SDL_StartTextInput();
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            SDL_StopTextInput();
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe {
            SDL_IsTextInputActive() == SDL_bool::SDL_TRUE
        }
    }

    pub fn set_area(&mut self,x : i32,y : i32,w : u32,h : u32) {
        let mut rect = SDL_Rect{
            x,
            y,
            w: w as _,
            h: h as _,
        };
        unsafe {
            SDL_SetTextInputRect(&mut rect)
        };
    }

    pub fn has_screen_keyboard_support(&self) -> bool {
        unsafe { SDL_HasScreenKeyboardSupport() == SDL_bool::SDL_TRUE }
    }

    pub fn is_screen_keyboard_shown(&self, window: &Window) -> bool {
        let window_ptr = unsafe { window.window_ptr() };
        return unsafe { SDL_IsScreenKeyboardShown(window_ptr) == SDL_bool::SDL_TRUE };
    }
}

