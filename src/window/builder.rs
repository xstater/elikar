extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::window::{Window, WindowId};
use std::ffi::{CString};
use crate::common::{ SdlError,Result };
use crate::window::manager::Manager;
use std::fmt::{Debug, Formatter};

pub struct Builder<'a>{
    title : String,
    x : i32,y : i32,
    w : i32,h : i32,
    flags : u32,
    manager : &'a mut Manager,
}

impl<'a> Debug for Builder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Builder")
            .field("title",&self.title)
            .field("x",&self.x)
            .field("y",&self.y)
            .field("w",&self.w)
            .field("h",&self.h)
            .field("flags",&self.flags)
            .finish()
    }
}

impl<'a> Builder<'a>{
    pub(in crate) fn from_manager(manager : &'a mut Manager) -> Builder<'a> {
        Builder {
            title: "elikar".to_string(),
            x : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            y : SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            w : 1280,
            h : 768,
            flags : SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            manager,
        }
    }

    pub fn title(mut self,text : &str) -> Self{
        self.title = text.to_owned();
        self
    }

    pub fn position(mut self,x : i32,y : i32) -> Self{
        self.x = x;
        self.y = y;
        self
    }

    pub fn position_default(mut self) -> Self{
        self.x = SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        self.y = SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        self
    }

    pub fn position_centred(mut self) -> Self{
        self.x = SDL_WINDOWPOS_CENTERED_MASK as i32;
        self.y = SDL_WINDOWPOS_CENTERED_MASK as i32;
        self
    }

    pub fn size(mut self,w : u32,h : u32) -> Self{
        self.w = w as i32;
        self.h = h as i32;
        self
    }

    pub fn fullscreen(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
        self
    }

    pub fn fullscreen_desktop(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        self
    }

    pub fn opengl(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
        self
    }
    pub fn vulkan(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
        self
    }
    pub fn hidden(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        self
    }
    pub fn borderless(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;
        self
    }
    pub fn resizable(mut self) -> Self {
        self.flags |= SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
        self
    }
    pub fn minimized(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32;
        self
    }
    pub fn maximized(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32;
        self
    }
    pub fn input_grabbed(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32;
        self
    }
    pub fn input_focus(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32;
        self
    }
    pub fn mouse_focus(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MOUSE_FOCUS as u32;
        self
    }
    pub fn allow_high_dpi(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        self
    }
    pub fn mouse_capture(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_MOUSE_CAPTURE as u32;
        self
    }
    pub fn always_on_top(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32;
        self
    }
    pub fn skip_taskbar(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_SKIP_TASKBAR as u32;
        self
    }
    pub fn utility(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_UTILITY as u32;
        self
    }
    pub fn tooltip(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_TOOLTIP as u32;
        self
    }
    pub fn popup_menu(mut self) -> Self{
        self.flags |= SDL_WindowFlags::SDL_WINDOW_POPUP_MENU as u32;
        self
    }

    pub fn build(self) -> Result<&'a mut Window>{
        let title_str = CString::new(self.title.clone()).unwrap();
        let window_ptr : *mut SDL_Window = unsafe {
            SDL_CreateWindow(
                title_str.as_ptr(),
                self.x,self.y,
                self.w,self.h,
                self.flags)
        };
        if window_ptr.is_null() {
            return Err(SdlError::get())
        } else {
            let id = unsafe { SDL_GetWindowID(window_ptr) };
            if id == 0 {
                return Err(SdlError::get())
            }
            let id = WindowId::from_u32(id);
            let window = unsafe { Window::from_ptr(id,window_ptr) };
            self.manager.add_window(window);
            Ok(self.manager.window_mut(id).unwrap())
        }
    }

}

