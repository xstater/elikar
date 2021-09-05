use sdl2_sys::*;
use std::path::Path;
use crate::sdl_renderer::Renderer;
use crate::common::SdlError;
use crate::common::Result;
use std::ffi::CString;
use crate::sdl_renderer::rect::Rect;
use crate::sdl_renderer::point::Point;
use sdl2_sys::sys::*;

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

    pub fn copy_ex(&mut self,
                   renderer : &Renderer,
                   dst : Rect,
                   center : Point,
                   flip_horizontal : bool,
                   flip_vertical : bool
    ) -> Result<()> {
        let (x,y,w,h) =
            (self.position.0, self.position.1, self.size.0, self.size.1);
        let src = Rect::new(x,y,w,h);
        let flip = unsafe {
            match (flip_horizontal, flip_vertical) {
                (false,false) => SDL_FLIP_NONE,
                (true,false) => SDL_FLIP_HORIZONTAL,
                (false,true) => SDL_FLIP_VERTICAL,
                (true,true) => transmute::<u32, sys::SDL_RendererFlip>(
                            transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_HORIZONTAL) |
                               transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_VERTICAL)
                )
            }
        };

        let ret = unsafe {
            SDL_RenderCopyEx(renderer.sdl_renderer,
                             self.texture,
                             match src.into() {
                                 Some(ref rect) => rect.raw(),
                                 None => ptr::null(),
                             },
                             match dst.into() {
                                 Some(ref rect) => rect.raw(),
                                 None => ptr::null(),
                             },
                             angle as c_double,
                             match center.into() {
                                 Some(ref point) => point.raw(),
                                 None => ptr::null(),
                             },
                             flip
            )
        };

        if ret != 0 { Err(get_error()) } else { Ok(()) }

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


pub struct ExInfo {
    dst_rect : Rect,
    angle : f64,
    center : Point,
    flip_horizontal : bool,
    flip_vertical : bool
}

unsafe impl Send for Sprite {}
unsafe impl Sync for Sprite {}

impl ExInfo {
    pub fn new(dst_rect : Rect,
               angle : f64,
               center : Point,
               flip_horizontal : bool,
               flip_vertical : bool
    ) -> Self {
        ExInfo {
            dst_rect,
            angle,
            center,
            flip_horizontal,
            flip_vertical
        }
    }

    pub fn dst(&self) -> Rect {
        self.dst_rect
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn flip(&self) -> (bool,bool) {
        (self.flip_horizontal,self.flip_vertical)
    }

    pub fn set_dst(&mut self,dst : Rect) {
        self.dst_rect = dst;
    }

    pub fn set_angle(&mut self,angle : f64) {
        self.angle = angle;
    }

    pub fn set_center(&mut self,center : Point) {
        self.center = center;
    }

    pub fn set_flip(&mut self,(flip_h,flip_v) : (bool,bool)) {
        self.flip_horizontal = flip_h;
        self.flip_vertical = flip_v;
    }
}