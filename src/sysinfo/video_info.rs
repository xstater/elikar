extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use std::os::raw::{c_char, c_int};
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::ptr::null_mut;

pub struct VideoInfo{}

#[derive(Debug,Default,Copy,Clone,PartialOrd,PartialEq)]
pub struct  DPI{
    ddpi : f32,
    hdpi : f32,
    vdpi : f32
}

#[derive(Debug,Default,Copy,Clone,PartialOrd,PartialEq)]
pub struct DisplayMode{
    size : (i32,i32),
    refresh_rate : u32
}

#[derive(Debug,Default,Clone)]
pub struct Screen{
    dpi : DPI,
    bound : (i32,i32,i32,i32),
    modes : Vec<DisplayMode>,
    name : String
}

impl VideoInfo{
    pub fn all_drivers_name(&self) -> Result<Vec<String>,String>{
        let num = unsafe { SDL_GetNumVideoDrivers() };
        if num < 0 {
            return Err(get_error());
        }
        let mut names : Vec<String> = Vec::with_capacity(num as usize);
        for i in 0..num {
            let cname : *const c_char = unsafe { SDL_GetVideoDriver(i as c_int) };
            names.push(unsafe {CStr::from_ptr(cname) }
                .to_str()
                .unwrap_or("Invalid UTF8 String")
                .to_owned());
        }
        Ok(names)
    }

    pub fn current_drivers_name(&self) -> String{
        let cname : *const c_char = unsafe { SDL_GetCurrentVideoDriver() };
        unsafe { CStr::from_ptr(cname) }
            .to_str()
            .unwrap_or("Invalid UTF8 Stting")
            .to_owned()
    }

    pub fn screens(&self) -> Result<Vec<Screen>,String>{
        let screen_num = unsafe { SDL_GetNumVideoDisplays() };
        if screen_num < 0 {
            return Err(get_error());
        }
        let mut screens : Vec<Screen> = Vec::with_capacity(screen_num as usize);
        for i in 0..screen_num{
            let mut screen : Screen = Screen::default();
            screen.name = unsafe { CStr::from_ptr(SDL_GetDisplayName(i)) }
                .to_str().unwrap_or("Invalid UTF8 String")
                .to_owned();

            let mut bound : SDL_Rect = SDL_Rect{
                x : 0,y : 0,w : 0,h : 0
            };
            if unsafe { SDL_GetDisplayBounds(i,&mut bound) } < 0 {
                return Err(get_error())
            }
            screen.bound.0 = bound.x;
            screen.bound.1 = bound.y;
            screen.bound.2 = bound.w;
            screen.bound.3 = bound.h;

            if unsafe { SDL_GetDisplayDPI(i,&mut screen.dpi.ddpi,&mut screen.dpi.hdpi,&mut screen.dpi.vdpi) } < 0 {
                return Err(get_error())
            }

            let mode_num = unsafe { SDL_GetNumDisplayModes(i) };
            for j in 0..mode_num{
                let mut sdlmode : SDL_DisplayMode = SDL_DisplayMode{
                    format : 0,w : 0,h : 0,refresh_rate : 0,driverdata : null_mut()
                };
                if unsafe { SDL_GetDisplayMode(i,j,&mut sdlmode) } < 0 {
                    return Err(get_error())
                }
                screen.modes.push(DisplayMode{
                    size: (sdlmode.w, sdlmode.h),
                    refresh_rate: sdlmode.refresh_rate as u32
                })
            }
            screens.push(screen);
        }
        Ok(screens)
    }
}

impl Display for DPI{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"(ddpi:{},hdpi:{},vdpi:{})",self.ddpi,self.hdpi,self.vdpi)
    }
}

impl Display for DisplayMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} x {} @ {}Hz",self.size.0,self.size.1,self.refresh_rate)
    }
}

impl Display for Screen{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Screen:\n\t{}\n\tbound:({},{},{},{})\n\t{}\n\tdisplay modes:\n",
            self.name,
            self.bound.0,
            self.bound.1,
            self.bound.2,
            self.bound.3,
            self.dpi) ?;
        for mode in &self.modes {
            write!(f,"\t\t{}\n",mode) ?;
        }
        Ok(())
    }
}