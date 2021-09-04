use sdl2_sys::*;
use std::path::Path;
use crate::sdl_renderer::Renderer;
use std::ptr::{null, null_mut};
use crate::common::SdlError;
use crate::common::Result;
use std::ffi::CString;

// !!!! 这里可能会有生命周期问题 ！！！
pub struct Sprite {
    texture : *mut SDL_Texture,
    position : (i32,i32),
    size : (u32,u32),
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
    pub fn from_bmp<P : AsRef<Path>>(renderer : &Renderer,path : P) -> Result<Sprite> {
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
            position : (0,0),
            size : (w as _,h as _)
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
}