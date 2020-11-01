mod video_info;

extern crate sdl2_sys;

use std::ffi::CStr;
use crate::sysinfo::video_info::VideoInfo;

pub struct SystemInfo{
    video_info : VideoInfo
}

impl SystemInfo {
    pub fn new() -> SystemInfo{
        SystemInfo{
            video_info : VideoInfo{}
        }
    }

    #[inline]
    pub fn get_platform(&self) -> &'static str{
        unsafe {
            CStr::from_ptr(sdl2_sys::SDL_GetPlatform()).to_str().unwrap()
        }
    }

    //bytes
    #[inline]
    pub fn get_cpu_cache_line_size(&self) -> u32{
        unsafe {
            sdl2_sys::SDL_GetCPUCacheLineSize() as u32
        }
    }

    #[inline]
    pub fn get_cpu_count(&self) -> u32{
        unsafe {
            sdl2_sys::SDL_GetCPUCount() as u32
        }
    }

    //mib
    #[inline]
    pub fn get_system_ram(&self) -> u32{
        unsafe {
            sdl2_sys::SDL_GetSystemRAM() as u32
        }
    }

    #[inline]
    pub fn has_3d_now(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_Has3DNow() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_avx(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasAVX() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_avx2(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasAVX2() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_alti_vec(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasAltiVec() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_mmx(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasMMX() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_rdtsc(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasRDTSC() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_sse(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasSSE() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse2(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasSSE2() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse3(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasSSE3() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse41(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasSSE41() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse42(&self) -> bool{
        unsafe {
            sdl2_sys::SDL_HasSSE42() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    pub fn get_video_info(&self) -> &VideoInfo{
        &self.video_info
    }


}
