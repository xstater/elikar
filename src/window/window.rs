extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Weak, RwLock};
use crate::window::manager::ManagerBase;

///### safety:
///  Window have a raw pointer of window.
/// this pointer dangling when manager cannot get Arc pointer
#[derive(Clone)]
pub struct Window{
    manager : Weak<RwLock<ManagerBase>>,
    raw_window : *mut SDL_Window
}

unsafe impl Send for Window{}
unsafe impl Sync for Window{}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub enum Error{
    InvalidWindow,
    SDLError(String)
}

pub type Result<T> = std::result::Result<T, Error>;

impl Window {
    pub(in crate::window) unsafe fn from_ptr(manager : Weak<RwLock<ManagerBase>>,ptr : *mut SDL_Window) -> Window{
        Window{
            manager,
            raw_window : ptr
        }
    }

    pub(in crate) unsafe fn split(self) -> (Weak<RwLock<ManagerBase>>,*mut SDL_Window){
        (self.manager,self.raw_window)
    }

    pub unsafe fn raw_window(&self) -> *mut SDL_Window {
        self.raw_window
    }

    pub fn size(&self) -> Result<(u32,u32)> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        let (mut w,mut h) = (0,0);
        unsafe {
            SDL_GetWindowSize(self.raw_window,&mut w as *mut i32,&mut h as *mut i32);
        }
        Ok((w as u32,h as u32))
    }

    pub fn set_size(&mut self,w : u32,h : u32) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_SetWindowSize(self.raw_window,w as i32,h as i32)
        })
    }

    pub fn position(&self) -> Result<(u32,u32)>{
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        let (mut x,mut y) = (0,0);
        unsafe {
            SDL_GetWindowPosition(self.raw_window,&mut x as *mut i32,&mut y as *mut i32);
        }
        Ok((x as u32,y as u32))
    }

    pub fn set_position(&mut self,x : u32,y : u32) -> Result<()>{
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_SetWindowPosition(self.raw_window,x as i32,y as i32)
        })
    }

    pub fn brightness(&self) -> Result<f32> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_GetWindowBrightness(self.raw_window)
        })
    }

    pub fn set_brightness(&mut self,bright : f32) -> Result<()>{
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        let err = unsafe {
            SDL_SetWindowBrightness(self.raw_window,bright)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(Error::SDLError(get_error()))
        }
    }

    pub fn id(&self) -> Result<u32>{
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        let id = unsafe { SDL_GetWindowID(self.raw_window) };
        if id != 0 {
            Ok(id)
        }else{
            Err(Error::SDLError(get_error()))
        }
    }

    pub fn opacity(&self) -> Result<f32> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        let mut op = 0.0 as f32;
        if unsafe { SDL_GetWindowOpacity(self.raw_window,&mut op as *mut f32) } == 0 {
            Ok(op)
        } else {
            Err(Error::SDLError(get_error()))
        }
    }
    pub fn set_opacity(&mut self,opa : f32) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        let err = unsafe {
            SDL_SetWindowOpacity(self.raw_window,opa)
        };
        if err == 0 {
            Ok(())
        }else{
            Err(Error::SDLError(get_error()))
        }
    }

    pub fn title(&self) -> Result<String> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.read().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            let ttl = SDL_GetWindowTitle(self.raw_window);
            //unwrap here: because the valid of the title string is granted by SDL
            CStr::from_ptr(ttl as *const _).to_str().unwrap().to_owned()
        })
    }

    pub fn set_title(&mut self,ttl : &str) -> Result<()>{
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        //unwrap here:because the ttl is a valid Rust UTF8 String
        //there is no possibility to failure
        let ttl_str = CString::new(ttl).unwrap();
        let ptr = ttl_str.as_ptr() as *const c_char;
        Ok(unsafe {
            SDL_SetWindowTitle(self.raw_window,ptr);
        })
    }

    pub fn hide(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_HideWindow(self.raw_window)
        })
    }

    pub fn show(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_ShowWindow(self.raw_window)
        })
    }

    pub fn maximize(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_MaximizeWindow(self.raw_window)
        })
    }

    pub fn minimize(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_MinimizeWindow(self.raw_window)
        })
    }

    pub fn raise(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_RaiseWindow(self.raw_window)
        })
    }

    pub fn restore(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_RestoreWindow(self.raw_window)
        })
    }

    pub fn set_fullscreen(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        let err = unsafe {
            SDL_SetWindowFullscreen(self.raw_window,
                                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(Error::SDLError(get_error()))
        }
    }

    pub fn set_fullscreen_desktop(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        let err = unsafe {
            SDL_SetWindowFullscreen(self.raw_window,
                                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(Error::SDLError(get_error()))
        }
    }

    pub fn set_resizable(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_SetWindowResizable(self.raw_window,SDL_bool::SDL_TRUE)
        })
    }

    pub fn set_unresizable(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_SetWindowResizable(self.raw_window,SDL_bool::SDL_FALSE)
        })
    }

    pub fn gl_swap(&mut self) -> Result<()> {
        let ptr = self.manager.upgrade().ok_or(Error::InvalidWindow)?;
        let _guard = ptr.write().map_err(|_| Error::InvalidWindow)?;
        Ok(unsafe {
            SDL_GL_SwapWindow(self.raw_window)
        })
    }

    pub fn set_icon(&mut self) {
        unimplemented!()
    }
}

