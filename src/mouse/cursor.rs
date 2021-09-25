extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::{ SdlError,Result };

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
    pub(in crate::mouse) ptr : *mut SDL_Cursor
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { SDL_FreeCursor(self.ptr); }
    }
}

impl Cursor {
    pub fn default() -> Result<Cursor> {
        let ptr = unsafe { SDL_GetDefaultCursor() };
        if ptr.is_null() {
            Err(SdlError::get())
        }else{
            Ok(Cursor{ ptr })
        }
    }

    pub fn system(cursor : SystemCursor) -> Result<Cursor>{
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
            Err(SdlError::get())
        }else{
            Ok(Cursor { ptr })
        }
    }

}