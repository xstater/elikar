extern crate sdl2_sys;

use sdl2_sys::*;
use std::os::raw::{c_char, c_void, c_int};
use crate::common::{SdlError, Result, from_sdl_string};
use std::ffi::CString;

#[derive(Debug,Copy,Clone)]
pub struct Clipboard;

impl Clipboard {
    pub fn has(&self) -> bool {
        unsafe { SDL_HasClipboardText() == SDL_bool::SDL_TRUE }
    }

    pub fn set(&mut self,text: &str) -> Result<()> {
        // because text is a rust string
        // this unwrap cannot be failed
        let text = CString::new(text).unwrap();
        let res: c_int = unsafe { SDL_SetClipboardText(text.as_ptr()) };
        if res != 0 {
            Err(SdlError::get())
        } else {
            Ok(())
        }
    }

    pub fn get(&self) -> Result<String> {
        let str_ptr: *mut c_char = unsafe { SDL_GetClipboardText() };
        if str_ptr.is_null() {
            return Err(SdlError::get());
        }
        let text = unsafe { from_sdl_string(str_ptr) };
        unsafe { SDL_free(str_ptr as *mut c_void) };
        Ok(text)
    }
}