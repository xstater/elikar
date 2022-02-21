use sdl2_sys::*;
use xecs::resource::Resource;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;
pub use code::Code;

mod code;
pub mod events;

pub struct Keyboard {
    // To avoid be constructed by user
    _marker : PhantomData<()>
}

impl Resource for Keyboard {}

impl Keyboard {
    pub(in crate) fn new() -> Keyboard {
        Keyboard {
            _marker : Default::default()
        }
    }

    pub fn key_name(&self, code: Code) -> String {
        let str_ptr: *const c_char = unsafe { SDL_GetScancodeName(code.into()) };
        unsafe { CStr::from_ptr(str_ptr) }
            .to_str()
            .unwrap() //unwrap here: UTF8 validation was granted by SDL
            .to_owned()
    }

    pub fn mod_state(&self) -> Mod {
        Mod::new(unsafe { SDL_GetModState() as u32 })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub struct Mod(u32);

impl Mod {
    pub(in crate::keyboard) fn new(value: u32) -> Mod {
        Mod(value)
    }

    pub fn none(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_NONE as u32 == SDL_Keymod::KMOD_NONE as u32
    }

    pub fn left_shift(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_LSHIFT as u32 == SDL_Keymod::KMOD_LSHIFT as u32
    }

    pub fn right_shift(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_RSHIFT as u32 == SDL_Keymod::KMOD_RSHIFT as u32
    }

    pub fn shift(&self) -> bool {
        self.left_shift() || self.right_shift()
    }

    pub fn left_ctrl(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_LCTRL as u32 == SDL_Keymod::KMOD_LCTRL as u32
    }

    pub fn right_ctrl(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_RCTRL as u32 == SDL_Keymod::KMOD_RCTRL as u32
    }

    pub fn ctrl(&self) -> bool {
        self.left_ctrl() || self.right_ctrl()
    }

    pub fn left_alt(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_LALT as u32 == SDL_Keymod::KMOD_LALT as u32
    }

    pub fn right_alt(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_RALT as u32 == SDL_Keymod::KMOD_RALT as u32
    }

    pub fn alt(&self) -> bool {
        self.left_alt() || self.right_alt()
    }

    pub fn left_gui(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_LGUI as u32 == SDL_Keymod::KMOD_LGUI as u32
    }

    pub fn right_gui(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_RGUI as u32 == SDL_Keymod::KMOD_RGUI as u32
    }

    pub fn gui(&self) -> bool {
        self.left_gui() || self.right_gui()
    }

    pub fn num(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_NUM as u32 == SDL_Keymod::KMOD_NUM as u32
    }

    pub fn caps(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_CAPS as u32 == SDL_Keymod::KMOD_CAPS as u32
    }

    pub fn mode(&self) -> bool {
        self.0 & SDL_Keymod::KMOD_MODE as u32 == SDL_Keymod::KMOD_MODE as u32
    }
}

