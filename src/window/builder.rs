extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::window::Window;
use std::ffi::{CString};
use crate::common::get_error;
use std::sync::{Weak, RwLock};
use crate::window::manager::ManagerBase;

pub struct Builder{
    windows_manager : Weak<RwLock<ManagerBase>>,
    title : String,
    x : i32,y : i32,
    w : i32,h : i32,
    flags : u32
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub enum Error{
    InvalidManager,
    SDLError(String)
}

pub type Result<T> = std::result::Result<T,Error>;

impl Builder {
    pub(in crate::window) fn from_windows_manager_base(wm : Weak<RwLock<ManagerBase>>) -> Builder{
        Builder{
            windows_manager : wm,
            title : "elikar".to_owned(),
            x : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            y : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            w : 1280,
            h : 768,
            flags : SDL_WindowFlags::SDL_WINDOW_SHOWN as u32
        }
    }


    pub fn title(&mut self,text : &str) -> &mut Self{
        self.title = text.to_owned();
        self
    }

    pub fn position(&mut self,x : i32,y : i32) -> &mut Self{
        self.x = x;
        self.y = y;
        self
    }

    pub fn position_default(&mut self) -> &mut Self{
        self.x = SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        self.y = SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        self
    }

    pub fn position_centred(&mut self) -> &mut Self{
        self.x = SDL_WINDOWPOS_CENTERED_MASK as i32;
        self.y = SDL_WINDOWPOS_CENTERED_MASK as i32;
        self
    }

    pub fn size(&mut self,w : u32,h : u32) -> &mut Self{
        self.w = w as i32;
        self.h = h as i32;
        self
    }

    pub fn fullscreen(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
        self
    }

    pub fn fullscreen_desktop(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        self
    }

    pub fn opengl(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
        self
    }
    pub fn vulkan(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
        self
    }
    pub fn hidden(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        self
    }
    pub fn borderless(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;
        self
    }
    pub fn minimized(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32;
        self
    }
    pub fn maximized(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32;
        self
    }
    pub fn input_grabbed(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32;
        self
    }
    pub fn allow_high_dpi(&mut self) -> &mut Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        self
    }

    pub fn build(&self) -> Result<Window>{
        let ptr = self.windows_manager.upgrade().ok_or(Error::InvalidManager)?;
        let mut guard = ptr.write().map_err(|_|Error::InvalidManager)?;
        let title_str = CString::new(self.title.clone()).unwrap();//safe here
        let window_ptr : *mut SDL_Window = unsafe {
            SDL_CreateWindow(
                title_str.as_ptr(),
                self.x,self.y,
                self.w,self.h,
                self.flags)
        };
        guard.windows.push(window_ptr);
        if window_ptr.is_null() {
            Err(Error::SDLError(get_error()))
        } else {
            Ok(unsafe {  Window::from_ptr(self.windows_manager.clone(),window_ptr) })
        }
    }

}

