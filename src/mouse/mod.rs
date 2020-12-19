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

pub fn global_position() -> (i32,i32) {
    let (mut x,mut y) = (0,0);
    unsafe{ SDL_GetGlobalMouseState(&mut x as *mut _,&mut y as *mut _) };
    (x,y)
}

pub fn is_relative() -> bool{
    unsafe{ SDL_GetRelativeMouseMode() == SDL_bool::SDL_TRUE}
}

pub fn enable_relative() -> Result<(),String> {
    if unsafe { SDL_SetRelativeMouseMode(SDL_bool::SDL_TRUE) } != 0 {
        Err(get_error())
    }else{
        Ok(())
    }
}

pub fn disable_relative() -> Result<(),String> {
    if unsafe { SDL_SetRelativeMouseMode(SDL_bool::SDL_FALSE) } != 0 {
        Err(get_error())
    }else{
        Ok(())
    }
}

pub fn relative_position() -> (i32,i32){
    let (mut x,mut y) = (0,0);
    unsafe{ SDL_GetRelativeMouseState(&mut x as *mut _,&mut y as *mut _) };
    (x,y)
}

pub fn warp_global(x : i32,y : i32) -> Result<(),String> {
    if unsafe{ SDL_WarpMouseGlobal(x,y) } != 0 {
        Err(get_error())
    }else{
        Ok(())
    }
}