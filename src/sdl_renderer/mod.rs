pub mod point;
pub mod system;
pub mod sprite;
pub mod rect;

use sdl2_sys::*;
use crate::window::Window;
use crate::common::{Result, SdlError};
use std::os::raw::c_int;
use xblend::{RGBA,rgba};

pub type Color = RGBA<u8>;

pub struct Renderer{
    sdl_renderer : *mut SDL_Renderer,
    clear_color : Color,
}

impl Renderer {
    pub unsafe fn from_ptr(ptr : *mut SDL_Renderer) -> Renderer {
        Renderer {
            sdl_renderer: ptr,
            clear_color : rgba!(0,0,0,255)
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut SDL_Renderer {
        self.sdl_renderer
    }

    pub fn builder() -> RendererBuilder {
        RendererBuilder{
            index: -1,
            flags: 0
        }
    }

    pub fn clear(&mut self,color : Color){
        self.clear_color = color;
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.sdl_renderer)
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct RendererBuilder {
    index : c_int,
    flags : u32
}

impl RendererBuilder {
    pub fn index(&mut self,index : i32) -> &mut Self{
        self.index = index;
        self
    }

    pub fn software(self) -> Self {
        RendererBuilder {
            flags: self.flags | SDL_RendererFlags::SDL_RENDERER_SOFTWARE as u32,
            .. self
        }
    }

    pub fn accelerated(self) -> Self {
        RendererBuilder {
            flags: self.flags | SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
                .. self
        }
    }

    pub fn vsync(self) -> Self {
        RendererBuilder {
            flags : self.flags | SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32,
            .. self
        }
    }

    pub fn target_texture(self) -> Self {
        RendererBuilder {
            flags : self.flags | SDL_RendererFlags::SDL_RENDERER_TARGETTEXTURE as u32,
            .. self
        }
    }

    pub fn build(self,window : &Window) -> Result<Renderer> {
        let renderer_ptr : *mut SDL_Renderer = unsafe {
            SDL_CreateRenderer(window.window_ptr(), self.index, self.flags)
        };
        if renderer_ptr.is_null() {
            Err(SdlError::get())
        } else {
            Ok(Renderer{
                sdl_renderer: renderer_ptr,
                clear_color: rgba!(0,0,0,255)
            })
        }
    }
}
