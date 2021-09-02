extern crate sdl2_sys;

use sdl2_sys::*;
use std::os::raw::{c_char, c_void, c_int};
use std::ffi::{CStr, CString};
use crate::common::{SdlError, Result, from_sdl_string};

pub fn has() -> bool {
    unsafe { SDL_HasClipboardText() == SDL_bool::SDL_TRUE }
}

pub fn set(text: &str) -> Result<()> {
    let cstr : &CStr = unsafe { CStr::from_ptr(text.as_ptr() as *const c_char) };
    let cstring : CString =  CString::from(cstr);
    let res : c_int = unsafe { SDL_SetClipboardText(cstring.as_ptr()) };
    if res != 0 {
        Err(SdlError::get())
    }else{
        Ok(())
    }
}

pub fn get() -> Result<String> {
    let str_ptr: *mut c_char = unsafe { SDL_GetClipboardText() };
    if str_ptr.is_null() {
        return Err(SdlError::get());
    }
    let text = unsafe { from_sdl_string(str_ptr) };
    unsafe { SDL_free(str_ptr as *mut c_void) };
    Ok(text)
}