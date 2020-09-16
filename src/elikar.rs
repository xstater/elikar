extern crate sdl2_sys;

use sdl2_sys::*;
use std::ffi::CStr;

pub struct Elikar{
    //nothing for now
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

pub fn get_error() -> String{
    unsafe {
        let err = SDL_GetError();
        CStr::from_ptr(err as *const _).to_str().unwrap().to_owned()
    }
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
        Ok(Elikar{})
    }
}

impl Drop for Elikar {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}
