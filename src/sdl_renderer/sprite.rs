use sdl2_sys::*;
use std::path::Path;
use crate::sdl_renderer::Renderer;
use std::ptr::{null, null_mut};
use crate::common::SdlError;
use crate::common::Result;
use std::ffi::CString;
use crate::sdl_renderer::rect::Rect;
use crate::sdl_renderer::point::Point;

pub struct Sprite {
    texture : *mut SDL_Texture,
    position : (i32,i32),
    size : (u32,u32),
    dst_rect : Option<Rect>,
    angle : f64,
    center : Option<Point>,
    flip_horizontal : bool,
    flip_vertical : bool
}

unsafe impl Send for Sprite {}
unsafe impl Sync for Sprite {}

impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.texture)
        }
    }
}

impl Sprite {
    pub fn from_bmp<P : AsRef<Path>>(renderer : &Renderer, path : P) -> Result<Sprite> {
        let path  = path.as_ref().to_str().unwrap();
        let path = CString::new(path).unwrap();
        let mode = CString::new("rb").unwrap();
        let file_ptr = unsafe { SDL_RWFromFile(path.as_ptr(),mode.as_ptr()) };
        if file_ptr.is_null() {
            return Err(SdlError::get());
        }
        let surface : *mut SDL_Surface = unsafe { SDL_LoadBMP_RW(file_ptr,1) };
        if surface.is_null() {
            return Err(SdlError::get());
        }
        let (w,h) = (unsafe { &*surface }.w, unsafe {&*surface}.h );
        let texture = unsafe { SDL_CreateTextureFromSurface(renderer.sdl_renderer,surface) };
        if texture.is_null() {
            return Err(SdlError::get());
        }
        unsafe { SDL_FreeSurface(surface) };
        Ok(Sprite{
            texture,
            position : (0, 0),
            size : (w as _,h as _),
            dst_rect: Option::None,
            angle: 0.0,
            center: Option::None,
            flip_horizontal: false,
            flip_vertical: false
        })
    }

    pub unsafe fn texture(&self) -> *mut SDL_Texture {
        self.texture
    }

    pub fn size(&self) -> (u32,u32) {
        self.size
    }

    pub fn set_size(&mut self,(w,h) : (u32,u32)) {
        self.size = (w,h);
    }

    pub fn position(&self) -> (i32,i32) {
        self.position
    }

    pub fn move_to(&mut self,(x,y) : (i32,i32)) {
        self.position = (x,y);
    }

    pub fn dst(&self) -> Option<Rect> {
        self.dst_rect
    }

    pub fn set_dst(&mut self,dst : Option<Rect>) {
        self.dst_rect = dst;
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self,angle : f64) {
        self.angle = angle;
    }

    pub fn center(&self) -> Option<Point> {
        self.center
    }

    pub fn set_center(&mut self,center : Option<Point>) {
        self.center = center;
    }

    pub fn flip(&self) -> (bool,bool) {
        (self.flip_horizontal,self.flip_vertical)
    }
    
    pub fn set_flip(&mut self,flip_h : bool,flip_v : bool) {
        self.flip_horizontal = flip_h;
        self.flip_vertical = flip_v;
    }
}