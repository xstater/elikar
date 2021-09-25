extern crate sdl2_sys;

pub mod cursor;
pub mod event;

use sdl2_sys::*;
use std::ptr::null_mut;
use crate::common::{ SdlError,Result };
use crate::mouse::cursor::Cursor;
use xecs::System;
use std::convert::Infallible;

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

    pub fn left(&self) -> bool{
        self.0 & LEFT_MASK == LEFT_MASK
    }
    pub fn right(&self) -> bool{
        self.0 & RIGHT_MASK == RIGHT_MASK
    }
    pub fn middle(&self) -> bool{
        self.0 & MIDDLE_MASK == MIDDLE_MASK
    }
    pub fn x1(&self) -> bool{
        self.0 & X1_MASK == X1_MASK
    }
    pub fn x2(&self) -> bool{
        self.0 & X2_MASK == X2_MASK
    }
}

pub struct Mouse{

}

impl Mouse {
    pub(in crate) fn new() -> Mouse {
        Mouse{}
    }

    pub fn capture(&mut self) -> Result<()> {
        let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_TRUE) };
        if errcode == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn release(&mut self) -> Result<()> {
        let errcode = unsafe { SDL_CaptureMouse(SDL_bool::SDL_FALSE) };
        if errcode == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn button(&self) -> ButtonState {
        ButtonState::new(unsafe { SDL_GetMouseState(null_mut(), null_mut()) })
    }

    pub fn global_position(&self) -> (i32, i32) {
        let (mut x, mut y) = (0, 0);
        unsafe { SDL_GetGlobalMouseState(&mut x as *mut _, &mut y as *mut _) };
        (x, y)
    }

    pub fn is_relative(&self) -> bool {
        unsafe { SDL_GetRelativeMouseMode() == SDL_bool::SDL_TRUE }
    }

    pub fn enable_relative(&mut self) -> Result<()> {
        if unsafe { SDL_SetRelativeMouseMode(SDL_bool::SDL_TRUE) } != 0 {
            Err(SdlError::get())
        } else {
            Ok(())
        }
    }

    pub fn disable_relative(&mut self) -> Result<()> {
        if unsafe { SDL_SetRelativeMouseMode(SDL_bool::SDL_FALSE) } != 0 {
            Err(SdlError::get())
        } else {
            Ok(())
        }
    }

    pub fn relative_position(&self) -> (i32, i32) {
        let (mut x, mut y) = (0, 0);
        unsafe { SDL_GetRelativeMouseState(&mut x as *mut _, &mut y as *mut _) };
        (x, y)
    }

    pub fn warp_global(&mut self,x: i32, y: i32) -> Result<()> {
        if unsafe { SDL_WarpMouseGlobal(x, y) } != 0 {
            Err(SdlError::get())
        } else {
            Ok(())
        }
    }

    pub fn show(&mut self){
        unsafe{
            SDL_ShowCursor(SDL_ENABLE as i32);
        }
    }

    pub fn hide(&mut self){
        unsafe{
            SDL_ShowCursor(SDL_DISABLE as i32);
        }
    }

    pub fn is_visible(&self) -> bool{
        unsafe {
            SDL_ShowCursor(SDL_QUERY) == SDL_ENABLE as i32
        }
    }

    pub fn set_cursor(&mut self,cursor : Cursor){
        unsafe{
            SDL_SetCursor(cursor.ptr);
        }
    }

    pub fn is_no_mouse(&self) -> bool{
        unsafe{SDL_GetCursor()}.is_null()
    }
}

impl<'a> System<'a> for Mouse {
    type InitResource = ();
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;
}