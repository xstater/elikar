extern crate sdl2_sys;

use std::ffi::CStr;
use xrunits::data::{Mebibyte, BuildMebibyte};

#[derive(Debug)]
pub struct PlatformInfo{}

impl PlatformInfo {
    pub(in crate::sysinfo) fn new() -> PlatformInfo {
        PlatformInfo {}
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
        }
    }
    #[inline]
    pub fn system_ram(&self) -> Mebibyte {
        unsafe {
            sdl2_sys::SDL_GetSystemRAM() as u32
        }.mib()
    }
}