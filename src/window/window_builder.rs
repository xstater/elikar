extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::window::Window;
use std::ffi::{CString};
use crate::common::get_error;
use crate::Elikar;

pub struct WindowBuilder{
    title : String,
    x : i32,y : i32,
    w : i32,h : i32,
    flags : u32
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder{
        WindowBuilder{
            title : "elikar".to_owned(),
            x : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            y : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            w : 1280,
            h : 768,
            flags : SDL_WindowFlags::SDL_WINDOW_SHOWN as u32
        }
    }

    pub fn title(self,text : &str) -> Self{
        Self{
            title : text.to_owned(),
            ..self
        }
    }

    pub fn position(self,x : i32,y : i32) -> Self{
        Self{
            x,y,
            ..self
        }
    }

    pub fn position_default(self) -> Self{
        Self{
            x : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            y : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            ..self
        }
    }

    pub fn position_centred(self) -> Self{
        Self{
            x : SDL_WINDOWPOS_CENTERED_MASK as i32,
            y : SDL_WINDOWPOS_CENTERED_MASK as i32,
            ..self
        }
    }

    pub fn size(self,w : u32,h : u32) -> Self{
        Self{
            w : w as i32,
            h : h as i32,
            ..self
        }
    }

    pub fn fullscreen(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32),
            ..self
        }
    }

    pub fn fullscreen_desktop(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32),
            ..self
        }
    }

    pub fn opengl(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_OPENGL as u32),
            ..self
        }
    }
    pub fn vulkan(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_VULKAN as u32),
            ..self
        }
    }
    pub fn hidden(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32),
            ..self
        }
    }
    pub fn borderless(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32),
            ..self
        }
    }
    pub fn minimized(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32),
            ..self
        }
    }
    pub fn maximized(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32),
            ..self
        }
    }
    pub fn input_grabbed(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32),
            ..self
        }
    }
    pub fn allow_high_dpi(self) -> Self{
        Self{
            flags : self.flags | (SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32),
            ..self
        }
    }

    pub fn build(self,_ : &Elikar) -> Result<Window,String>{
        let title_str = CString::new(self.title)
            .map_err(|_| "Invalid Title") ?;
        let window_ptr : *mut SDL_Window = unsafe {
            SDL_CreateWindow(
                title_str.as_ptr(),
                self.x,self.y,
                self.w,self.h,
                self.flags)
        };

        if window_ptr.is_null() {
            Err(get_error())
        } else {
            Ok(unsafe {  Window::from_ptr(window_ptr) })
        }
    }

}

