extern crate sdl2_sys;

pub mod button;
pub mod motion;
pub mod wheel;
pub mod cursor;

use sdl2_sys::*;
use std::ptr::null_mut;
use crate::common::get_error;
use crate::mouse::button::ButtonState;

pub fn capture() -> Result<(),String>{
    let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_TRUE) };
    if errcode == 0 {
        Ok(())
    }else{
        Err(get_error())
    }
}

pub fn release() -> Result<(),String>{
    let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_FALSE) };
    if errcode == 0 {
        Ok(())
    }else{
        Err(get_error())
    }
}

pub fn button() -> ButtonState{
    ButtonState::new(unsafe{ SDL_GetMouseState(null_mut(),null_mut()) })
}