extern crate sdl2_sys;

use xrunits::data::{Byte, BuildByte};

#[derive(Debug,Copy, Clone)]
pub struct CPUInfo;

impl CPUInfo {
    #[inline]
    pub fn cpu_cache_line_size(&self) -> Byte {
        unsafe {
            sdl2_sys::SDL_GetCPUCacheLineSize() as u32
        }.byte()
    }
    #[inline]
    pub fn cpu_count(&self) -> u32 {
        unsafe {
            sdl2_sys::SDL_GetCPUCount() as u32
        }
    }
    #[inline]
    pub fn has_3d_now(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_Has3DNow() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_avx(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasAVX() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_avx2(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasAVX2() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }

    #[inline]
    pub fn has_alti_vec(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasAltiVec() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_mmx(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasMMX() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_rdtsc(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasRDTSC() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }


    #[inline]
    pub fn has_sse(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasSSE() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse2(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasSSE2() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse3(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasSSE3() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse41(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasSSE41() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
    #[inline]
    pub fn has_sse42(&self) -> bool {
        unsafe {
            sdl2_sys::SDL_HasSSE42() == sdl2_sys::SDL_bool::SDL_TRUE
        }
    }
}