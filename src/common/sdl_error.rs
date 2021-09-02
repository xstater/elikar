extern crate sdl2_sys;

use sdl2_sys::{ SDL_GetError };
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::common::from_sdl_string;

#[derive(Debug,Default,Clone,Hash,Eq,PartialEq)]
pub struct SdlError(String);

pub type Result<T> = std::result::Result<T,SdlError>;

impl SdlError {
    pub fn get() -> SdlError {
        unsafe {
            let err_msg = SDL_GetError();
            SdlError(from_sdl_string(err_msg))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for SdlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"SDL error:{}",self.0)
    }
}

impl Error for SdlError {}
