extern crate sdl2_sys;

use std::ffi::CStr;
use xrunits::data::{Mebibyte, BuildMebibyte};

#[inline]
pub fn name()-> &'static str{
    unsafe {
        CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
    }
}
#[inline]
pub fn system_ram() -> Mebibyte{
    unsafe {
        sdl2_sys::SDL_GetSystemRAM() as u32
    }.mib()
}