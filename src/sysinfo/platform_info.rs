extern crate sdl2_sys;

use std::ffi::CStr;


pub struct PlatformInfo{}

impl PlatformInfo{
    #[inline]
    pub fn name(&self) -> &'static str{
        unsafe {
            CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
        }
    }
    //mib
    #[inline]
    pub fn system_ram(&self) -> u32{
        unsafe {
            sdl2_sys::SDL_GetSystemRAM() as u32
        }
    }
}