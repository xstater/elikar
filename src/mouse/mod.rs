extern crate sdl2_sys;

pub mod cursor;
pub mod event;

use sdl2_sys::*;
use std::ptr::null_mut;
use crate::common::get_error;

#[derive(Debug,Clone,Copy,Default,PartialOrd,PartialEq)]
pub struct ButtonState(u32);

const LEFT_MASK : u32 = 0x01;
const MIDDLE_MASK : u32 = 0x02;
const RIGHT_MASK : u32 = 0x04;
const X1_MASK : u32 = 0x08;
const X2_MASK : u32 = 0x10;

impl ButtonState {
    pub(in crate::mouse) fn new(value : u32) -> ButtonState{
        ButtonState(value)
    }

    pub fn is_left(&self) -> bool{
        self.0 & LEFT_MASK == LEFT_MASK
    }
    pub fn is_right(&self) -> bool{
        self.0 & RIGHT_MASK == RIGHT_MASK
    }
    pub fn is_middle(&self) -> bool{
        self.0 & MIDDLE_MASK == MIDDLE_MASK
    }
    pub fn is_x1(&self) -> bool{
        self.0 & X1_MASK == X1_MASK
    }
    pub fn is_x2(&self) -> bool{
        self.0 & X2_MASK == X2_MASK
    }
}

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