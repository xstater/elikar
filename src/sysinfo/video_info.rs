extern crate sdl2_sys;

use sdl2_sys::*;
use std::os::raw::{c_char, c_int};
use std::fmt::{Display, Formatter};
use std::ptr::null_mut;
use crate::common::{Result, SdlError, from_sdl_string};

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
        write!(f,"Screen:\n\tname:{}\n\tbound:({},{},{},{})\n\tdpi:{}\n\tdisplay modes:\n",
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

pub fn all_drivers_name() -> Result<Vec<String>>{
    let num = unsafe { SDL_GetNumVideoDrivers() };
    if num < 0 {
        return Err(SdlError::get());
    }
    let mut names : Vec<String> = Vec::with_capacity(num as usize);
    for i in 0..num {
        let cname : *const c_char = unsafe { SDL_GetVideoDriver(i as c_int) };
        names.push(unsafe { from_sdl_string(cname) })
    }
    Ok(names)
}

pub fn current_drivers_name() -> String {
    let cname : *const c_char = unsafe { SDL_GetCurrentVideoDriver() };
    unsafe { from_sdl_string(cname) }
}

pub fn screens() -> Result<Vec<Screen>>{
    let screen_num = unsafe { SDL_GetNumVideoDisplays() };
    if screen_num < 0 {
        return Err(SdlError::get());
    }
    let mut screens : Vec<Screen> = Vec::with_capacity(screen_num as usize);
    for i in 0..screen_num{
        let mut screen : Screen = Screen::default();
        screen.name = unsafe { from_sdl_string(SDL_GetDisplayName(i)) };

        let mut bound : SDL_Rect = SDL_Rect{
            x : 0,y : 0,w : 0,h : 0
        };
        if unsafe { SDL_GetDisplayBounds(i,&mut bound) } < 0 {
            return Err(SdlError::get())
        }
        screen.bound.0 = bound.x;
        screen.bound.1 = bound.y;
        screen.bound.2 = bound.w;
        screen.bound.3 = bound.h;

        if unsafe { SDL_GetDisplayDPI(i,&mut screen.dpi.ddpi,&mut screen.dpi.hdpi,&mut screen.dpi.vdpi) } < 0 {
            return Err(SdlError::get())
        }

        let mode_num = unsafe { SDL_GetNumDisplayModes(i) };
        for j in 0..mode_num{
            let mut sdlmode : SDL_DisplayMode = SDL_DisplayMode{
                format : 0,w : 0,h : 0,refresh_rate : 0,driverdata : null_mut()
            };
            if unsafe { SDL_GetDisplayMode(i,j,&mut sdlmode) } < 0 {
                return Err(SdlError::get())
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

