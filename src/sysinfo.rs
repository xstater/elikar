extern crate sdl2_sys;

use std::ffi::CStr;

#[inline]
pub fn get_platform() -> &'static str{
    unsafe {
        CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
    }
}

#[inline]
pub fn get_cpu_cache_line_size() -> i32{
    unsafe {
        sdl2_sys::SDL_GetCPUCacheLineSize()
    }
}

#[inline]
pub fn get_cpu_count() -> i32{
    unsafe {
        sdl2_sys::SDL_GetCPUCount()
    }
}

#[inline]
pub fn get_system_ram() -> i32{
    unsafe {
        sdl2_sys::SDL_GetSystemRAM()
    }
}