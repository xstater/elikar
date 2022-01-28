use std::{ffi::CStr, marker::PhantomData};
use xrunits::data::{BuildMebibyte, Mebibyte};

#[derive(Debug)]
pub struct PlatformInfo {
    _marker : PhantomData<()>
}

impl PlatformInfo {
    pub(in crate::sysinfo) fn new() -> PlatformInfo {
        PlatformInfo {
            _marker : Default::default()
        }
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(sdl2_sys::SDL_GetPlatform())
                .to_str()
                .unwrap()
        }
    }
    #[inline]
    pub fn system_ram(&self) -> Mebibyte {
        unsafe { sdl2_sys::SDL_GetSystemRAM() as u32 }.mib()
    }
}
