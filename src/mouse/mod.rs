extern crate sdl2_sys;

pub mod button;
pub mod motion;
pub mod wheel;

use sdl2_sys::*;
use std::ptr::null_mut;
use crate::common::get_error;
use crate::mouse::button::ButtonInfo;
use crate::mouse::motion::MotionInfo;

pub struct Mouse{
}

pub struct ButtonState(u32);

const LEFT_MASK : u32 = 0x01;
const MIDDLE_MASK : u32 = 0x02;
const RIGHT_MASK : u32 = 0x04;
const X1_MASK : u32 = 0x08;
const X2_MASK : u32 = 0x10;

impl ButtonState {
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

impl Mouse{
    pub fn capture(&mut self) -> Result<(),String>{
        let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_TRUE) };
        if errcode == 0 {
            Ok(())
        }else{
            Err(get_error())
        }
    }

    pub fn release(&mut self) -> Result<(),String>{
        let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_FALSE) };
        if errcode == 0 {
            Ok(())
        }else{
            Err(get_error())
        }
    }

    pub fn button(&self) -> ButtonState{
        ButtonState(unsafe{ SDL_GetMouseState(null_mut(),null_mut()) })
    }
}
