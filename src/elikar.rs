extern crate sdl2_sys;

use std::collections::LinkedList;
use sdl2_sys::*;
use crate::common::get_error;
use crate::window::{Window, WindowBuilder};
use crate::clipboard::Clipboard;

pub struct Elikar{
    clipboard : Clipboard,
    windows_list : LinkedList<Window>
}

#[derive(Debug)]
pub enum SdlInitError{
    Timer(String),
    Audio(String),
    Video(String),
    Joystick(String),
    Haptic(String),
    GameController(String),
    Events(String)
}


impl Elikar {
    pub fn new() -> Result<Elikar,SdlInitError> {
        unsafe {
            // if SDL_InitSubSystem(SDL_INIT_TIMER) != 0 {
            //      return Err(SdlInitError::Timer(get_error()));
            // }
            // if SDL_InitSubSystem(SDL_INIT_AUDIO) != 0 {
            //     return Err(SdlInitError::Audio(get_error()));
            // }
            if SDL_InitSubSystem(SDL_INIT_VIDEO) != 0 {
                return Err(SdlInitError::Video(get_error()));
            }
            // if SDL_InitSubSystem(SDL_INIT_JOYSTICK) != 0 {
            //     return Err(SdlInitError::Joystick(get_error()));
            // }
            // if SDL_InitSubSystem(SDL_INIT_HAPTIC) != 0 {
            //     return Err(SdlInitError::Haptic(get_error()));
            // }
            // if SDL_InitSubSystem(SDL_INIT_GAMECONTROLLER) != 0 {
            //     return Err(SdlInitError::GameController(get_error()));
            // }
            if SDL_InitSubSystem(SDL_INIT_EVENTS) != 0 {
                return Err(SdlInitError::Events(get_error()));
            }
        }
        Ok(Elikar{
            clipboard : Clipboard::new(),
            windows_list: LinkedList::new()
        })
    }

    pub fn clipboard(&self) -> &Clipboard{
        &self.clipboard
    }

    pub fn clipboard_mut(&mut self) -> &mut Clipboard{
        &mut self.clipboard
    }

    pub fn window_builder(&mut self) -> WindowBuilder{
        WindowBuilder::new(&mut self.windows_list)
    }
}

impl Drop for Elikar {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}
