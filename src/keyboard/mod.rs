extern crate sdl2_sys;

use sdl2_sys::*;

mod code;
pub mod screen;
pub mod event;

pub use code::Code;
use std::os::raw::c_char;
use std::ffi::{CStr};

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash)]
pub struct Mod(u32);

impl Mod{
    pub(in crate::keyboard) fn new(value : u32) -> Mod{
        Mod(value)
    }

    pub fn none(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_NONE as u32 == SDL_Keymod::KMOD_NONE as u32
    }

    pub fn left_shift(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_LSHIFT as u32 == SDL_Keymod::KMOD_LSHIFT as u32
    }

    pub fn right_shift(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_RSHIFT as u32 == SDL_Keymod::KMOD_RSHIFT as u32
    }

    pub fn left_ctrl(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_LCTRL as u32 == SDL_Keymod::KMOD_LCTRL as u32
    }

    pub fn right_ctrl(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_RCTRL as u32 == SDL_Keymod::KMOD_RCTRL as u32
    }

    pub fn left_alt(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_LALT as u32 == SDL_Keymod::KMOD_LALT as u32
    }

    pub fn right_alt(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_RALT as u32 == SDL_Keymod::KMOD_RALT as u32
    }

    pub fn left_gui(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_LGUI as u32 == SDL_Keymod::KMOD_LGUI as u32
    }

    pub fn right_gui(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_RGUI as u32 == SDL_Keymod::KMOD_RGUI as u32
    }

    pub fn num(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_NUM as u32 == SDL_Keymod::KMOD_NUM as u32
    }

    pub fn caps(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_CAPS as u32 == SDL_Keymod::KMOD_CAPS as u32
    }

    pub fn mode(&self) -> bool{
        self.0 & SDL_Keymod::KMOD_MODE as u32 == SDL_Keymod::KMOD_MODE as u32
    }
}

pub fn name(code : Code) -> String{
    let str_ptr : *const c_char = unsafe { SDL_GetScancodeName(code.into()) };
    unsafe{CStr::from_ptr(str_ptr)}
        .to_str()
        .unwrap()//unwrap here: UTF8 validation was granted by SDL
        .to_owned()
}

pub fn mod_state() -> Mod{
    Mod::new(unsafe{
        SDL_GetModState() as u32
    })
}

