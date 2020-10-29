extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use std::os::raw::{c_char, c_void, c_int};
use std::ffi::{CStr, CString};

pub struct Clipboard{
    //nothing
}

impl Clipboard {
    pub fn new() -> Clipboard{
        Clipboard{

        }
    }

    pub fn has(&self) -> bool {
        unsafe { SDL_HasClipboardText() == SDL_bool::SDL_TRUE }
    }

    pub fn set(&mut self,text: &str) -> Result<(), String> {
        let cstr : &CStr = unsafe { CStr::from_ptr(text.as_ptr() as *const c_char) };
        let cstring : CString =  CString::from(cstr);
        let res : c_int = unsafe { SDL_SetClipboardText(cstring.as_ptr()) };
        if res != 0 {
            Err(get_error())
        }else{
            Ok(())
        }
    }

    pub fn get(&self) -> Result<String, String> {
        let str_ptr: *mut c_char = unsafe { SDL_GetClipboardText() };
        if str_ptr.is_null() {
            return Err(get_error());
        }
        let text = unsafe { CStr::from_ptr(str_ptr).to_str().unwrap().to_owned() };
        unsafe { SDL_free(str_ptr as *mut c_void) };
        Ok(text)
    }
}