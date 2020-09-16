extern crate sdl2_sys;

use std::ffi::CStr;

#[inline]
pub fn get_platform() -> &'static str{
    unsafe {
        CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
    }
}

pub type Bytes = u32;
#[inline]
pub fn get_cpu_cache_line_size() -> Bytes{
    unsafe {
        sdl2_sys::SDL_GetCPUCacheLineSize() as u32
    }
}

#[inline]
pub fn get_cpu_count() -> u32{
    unsafe {
        sdl2_sys::SDL_GetCPUCount() as u32
    }
}

pub type MiB = u32;
#[inline]
pub fn get_system_ram() -> MiB{
    unsafe {
        sdl2_sys::SDL_GetSystemRAM() as u32
    }
}

#[inline]
pub fn has_3d_now() -> bool{
    unsafe {
        sdl2_sys::SDL_Has3DNow() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}

#[inline]
pub fn has_avx() -> bool{
    unsafe {
        sdl2_sys::SDL_HasAVX() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}

#[inline]
pub fn has_avx2() -> bool{
    unsafe {
        sdl2_sys::SDL_HasAVX2() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}

#[inline]
pub fn has_alti_vec() -> bool{
    unsafe {
        sdl2_sys::SDL_HasAltiVec() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}


#[inline]
pub fn has_mmx() -> bool{
    unsafe {
        sdl2_sys::SDL_HasMMX() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}


#[inline]
pub fn has_rdtsc() -> bool{
    unsafe {
        sdl2_sys::SDL_HasRDTSC() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}


#[inline]
pub fn has_sse() -> bool{
    unsafe {
        sdl2_sys::SDL_HasSSE() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}
#[inline]
pub fn has_sse2() -> bool{
    unsafe {
        sdl2_sys::SDL_HasSSE2() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}
#[inline]
pub fn has_sse3() -> bool{
    unsafe {
        sdl2_sys::SDL_HasSSE3() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}
#[inline]
pub fn has_sse41() -> bool{
    unsafe {
        sdl2_sys::SDL_HasSSE41() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}
#[inline]
pub fn has_sse42() -> bool{
    unsafe {
        sdl2_sys::SDL_HasSSE42() == sdl2_sys::SDL_bool::SDL_TRUE
    }
}
