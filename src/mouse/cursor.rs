extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;


#[derive(Debug,Clone)]
pub enum CursorError {
    CreatingDefaultCursor(String),
    CreatingSystemCursor(String),
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub enum SystemCursor{
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    Hand
}

#[derive(Debug)]
pub struct Cursor {
    ptr : *mut SDL_Cursor
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { SDL_FreeCursor(self.ptr); }
    }
}

impl Cursor {
    pub fn default() -> Result<Cursor, CursorError> {
        let ptr = unsafe { SDL_GetDefaultCursor() };
        if ptr.is_null() {
            Err(CursorError::CreatingDefaultCursor(get_error()))
        }else{
            Ok(Cursor{ ptr })
        }
    }

    pub fn system(cursor : SystemCursor) -> Result<Cursor, CursorError>{
        let sdlcursor = match cursor{
            SystemCursor::Arrow => SDL_SystemCursor::SDL_SYSTEM_CURSOR_ARROW,
            SystemCursor::IBeam => SDL_SystemCursor::SDL_SYSTEM_CURSOR_IBEAM,
            SystemCursor::Wait => SDL_SystemCursor::SDL_SYSTEM_CURSOR_WAIT,
            SystemCursor::Crosshair => SDL_SystemCursor::SDL_SYSTEM_CURSOR_CROSSHAIR,
            SystemCursor::WaitArrow => SDL_SystemCursor::SDL_SYSTEM_CURSOR_WAITARROW,
            SystemCursor::SizeNWSE => SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZENWSE,
            SystemCursor::SizeNESW => SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZENESW,
            SystemCursor::SizeWE => SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZEWE,
            SystemCursor::SizeNS => SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZENS,
            SystemCursor::SizeAll => SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZEALL,
            SystemCursor::No => SDL_SystemCursor::SDL_SYSTEM_CURSOR_NO,
            SystemCursor::Hand => SDL_SystemCursor::SDL_SYSTEM_CURSOR_HAND
        };
        let ptr = unsafe { SDL_CreateSystemCursor(sdlcursor) };
        if ptr.is_null() {
            Err(CursorError::CreatingSystemCursor(get_error()))
        }else{
            Ok(Cursor { ptr })
        }
    }

    pub fn set_as_cursor(&mut self){
        unsafe{
            SDL_SetCursor(self.ptr);
        }
    }
}

pub fn show(){
    unsafe{
        SDL_ShowCursor(SDL_ENABLE as i32);
    }
}

pub fn hide(){
    unsafe{
        SDL_ShowCursor(SDL_DISABLE as i32);
    }
}

pub fn is_visible() -> bool{
    unsafe {
        SDL_ShowCursor(SDL_QUERY) == SDL_ENABLE as i32
    }
}

pub fn is_no_mouse() -> bool{
    unsafe{SDL_GetCursor()}.is_null()
}