extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Debug)]
pub struct Window{
    raw_window : *mut SDL_Window
}

impl Window {
    pub fn new(title : &str, x : u32, y : u32,w : u32,h : u32) -> Result<Window,String>{
        let title_str = CString::new(title)
            .map_err(|_| "Invalid Title") ?;
        let window_ptr : *mut SDL_Window = unsafe {
            SDL_CreateWindow(
                title_str.as_ptr() as *const c_char,
                x as i32,y as i32,
                w as i32,h as i32,
                SDL_WindowFlags::SDL_WINDOW_OPENGL as u32)
        };

        if window_ptr.is_null() {
            Err(get_error())
        } else {
            Ok(Window {
                raw_window : window_ptr
            })
        }
    }

    pub fn size(&self) -> (u32,u32) {
        let (mut w,mut h) = (0,0);
        unsafe {
            SDL_GetWindowSize(self.raw_window,&mut w as *mut i32,&mut h as *mut i32);
        }
        (w as u32,h as u32)
    }

    pub fn set_size(&mut self,w : u32,h : u32){
        unsafe {
            SDL_SetWindowSize(self.raw_window,w as i32,h as i32)
        };
    }

    pub fn position(&self) -> (u32,u32) {
        let (mut x,mut y) = (0,0);
        unsafe {
            SDL_GetWindowPosition(self.raw_window,&mut x as *mut i32,&mut y as *mut i32);
        }
        (x as u32,y as u32)
    }

    pub fn set_position(&mut self,x : u32,y : u32){
        unsafe {
            SDL_SetWindowPosition(self.raw_window,x as i32,y as i32);
        }
    }

    pub fn brightness(&self) -> f32 {
        unsafe {
            SDL_GetWindowBrightness(self.raw_window)
        }
    }

    pub fn set_brightness(&mut self,bright : f32) -> Result<(),String>{
        let err = unsafe {
            SDL_SetWindowBrightness(self.raw_window,bright)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn id(&self) -> Result<u32,String>{
        let code = unsafe {
            SDL_GetWindowID(self.raw_window)
        };

        if code == 0 {
            Err(get_error())
        } else {
            Ok(code)
        }
    }

    pub fn opacity(&self) -> Result<f32,String> {
        let mut op = 0.0 as f32;
        if unsafe { SDL_GetWindowOpacity(self.raw_window,&mut op as *mut f32) } == 0 {
            Ok(op)
        } else {
            Err(get_error())
        }
    }
    pub fn set_opacity(&mut self,opa : f32) -> Result<(),String> {
        let err = unsafe {
            SDL_SetWindowOpacity(self.raw_window,opa)
        };
        if err == 0 {
            Ok(())
        }else{
            Err(get_error())
        }
    }

    pub fn title(&self) -> String {
        unsafe {
            let ttl = SDL_GetWindowTitle(self.raw_window);
            CStr::from_ptr(ttl as *const _).to_str().unwrap().to_owned()
        }
    }

    pub fn set_title(&mut self,ttl : &str) -> Result<(),String>{
        let ttl_str = CString::new(ttl)
            .map_err(|_| "Invalid Title") ?;
        let ptr = ttl_str.as_ptr() as *const c_char;
        unsafe {
            SDL_SetWindowTitle(self.raw_window,ptr);
        }
        Ok(())
    }

    pub fn hide(&mut self){
        unsafe {
            SDL_HideWindow(self.raw_window);
        }
    }

    pub fn show(&mut self){
        unsafe {
            SDL_ShowWindow(self.raw_window);
        }
    }

    pub fn maximize(&mut self){
        unsafe {
            SDL_MaximizeWindow(self.raw_window);
        }
    }

    pub fn minimize(&mut self){
        unsafe {
            SDL_MinimizeWindow(self.raw_window);
        }
    }

    pub fn raise(&mut self){
        unsafe {
            SDL_RaiseWindow(self.raw_window);
        }
    }

    pub fn restore(&mut self){
        unsafe {
            SDL_RestoreWindow(self.raw_window);
        }
    }

    pub fn set_fullscreen(&mut self) -> Result<(),String> {
        let err = unsafe {
            SDL_SetWindowFullscreen(self.raw_window,
                                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn set_fullscreen_desktop(&mut self) -> Result<(),String> {
        let err = unsafe {
            SDL_SetWindowFullscreen(self.raw_window,
                                    SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn set_resizable(&mut self){
        unsafe {
            SDL_SetWindowResizable(self.raw_window,SDL_bool::SDL_TRUE);
        }
    }

    pub fn set_unresizable(&mut self){
        unsafe {
            SDL_SetWindowResizable(self.raw_window,SDL_bool::SDL_FALSE);
        }
    }

    pub fn set_icon(&mut self) {
        unimplemented!()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.raw_window);
        }
    }
}

