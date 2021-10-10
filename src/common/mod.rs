mod sdl_error;
mod sparse_set;

pub use sdl_error::{Result, SdlError};
pub use sparse_set::SparseSet;
use std::ffi::CStr;
use std::os::raw::c_char;

/// ## Safety
/// sdl_string must be valid.
/// ## Panics
/// Panic if sdl_string is not a valid string.
pub(in crate) unsafe fn from_sdl_string(sdl_string: *const c_char) -> String {
    CStr::from_ptr(sdl_string).to_str().unwrap().to_owned()
}
