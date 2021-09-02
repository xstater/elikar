mod sdl_error;

pub use sdl_error::{SdlError,Result};
use std::os::raw::c_char;
use std::ffi::CStr;

/// ## Safety
/// sdl_string must be valid.
/// ## Panics
/// Panic if sdl_string is not a valid string.
pub(in crate) unsafe fn from_sdl_string(sdl_string : *const c_char) -> String {
    CStr::from_ptr(sdl_string)
        .to_str()
        .unwrap()
        .to_owned()
}
