extern crate sdl2_sys;

use std::ffi::CStr;
use crate::common::unit::{Data, Mebibyte};


pub struct PlatformInfo{}

impl PlatformInfo{
    #[inline]
    pub fn name(&self) -> &'static str{
        unsafe {
            CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
        }
    }
    #[inline]
    pub fn system_ram(&self) -> Data{
        unsafe {
            sdl2_sys::SDL_GetSystemRAM() as u32
        }.mb()
    }
}