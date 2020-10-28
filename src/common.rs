use sdl2_sys::*;
use std::ffi::CStr;

pub fn get_error() -> String{
    unsafe {
        let err = SDL_GetError();
        CStr::from_ptr(err as *const _).to_str().unwrap().to_owned()
    }
}