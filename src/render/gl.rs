use sdl2_sys::*;
use elikar_gl as gl;
use std::ffi::{c_void, CString};
use crate::render::{RendererContext, VSyncType};
use crate::window::Window;
use crate::common::{Result,SdlError};

pub struct GLContext {
    gl_context : *mut c_void,
    drawable_size : (u32,u32)
}

impl GLContext {
    pub fn from_window(window: &mut Window) -> Result<Self> {
        let gl_context = unsafe {
            let window = window.window_ptr();
            let gc = SDL_GL_CreateContext(window);
            if gc.is_null() {
                return Err(SdlError::get());
            }
            gc
        };
        gl::load_with(|s| unsafe{
            let s = CString::new(s).unwrap();
            SDL_GL_GetProcAddress(s.as_ptr() as *const _)
        } as *const _);
        let drawable_size = unsafe {
            let window = window.window_ptr();
            let mut width = 0;
            let mut height = 0;
            SDL_GL_GetDrawableSize(window,&mut width as *mut _,&mut height as *mut _);
            (width as u32,height as u32)
        };
        Ok(GLContext {
            gl_context,
            drawable_size
        })
    }
}

impl RendererContext for GLContext {
    fn drawable_size(&self) -> (u32, u32) {
        self.drawable_size
    }

    fn vsync(&self) -> VSyncType {
        let vsync_res = unsafe {
            SDL_GL_GetSwapInterval()
        };
        match vsync_res {
            1 => VSyncType::VSync,
            _ => VSyncType::None,
        }
    }

    fn set_vsync(&mut self,vsync_type : VSyncType) {
        let num = match vsync_type {
            VSyncType::None => 0,
            VSyncType::VSync => 1,
            VSyncType::AdaptiveVSync => -1
        };
        // ignore the Error
        unsafe {
            SDL_GL_SetSwapInterval(num);
        }
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.gl_context);
        }
    }
}