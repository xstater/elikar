use crate::common::{from_sdl_string, Result, SdlError};
use sdl2_sys::*;
use std::ffi::CString;

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct WindowId(u32);

impl WindowId {
    pub(in crate) fn from_u32(id: u32) -> WindowId {
        WindowId(id)
    }
}

pub struct Window {
    id: WindowId,
    ptr: *mut SDL_Window,
}

/// ## Safety:
/// Only window has this raw pointer.
unsafe impl Send for Window {}
/// ## Safety:
/// Window does not use any interior mutability;
unsafe impl Sync for Window {}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { SDL_DestroyWindow(self.ptr) }
    }
}

impl Window {
    /// ## Safety
    /// ptr must be a valid pointer
    pub(in crate) unsafe fn from_ptr(id: WindowId, ptr: *mut SDL_Window) -> Window {
        Window { id, ptr }
    }

    /// ## Safety
    /// Ptr must be ensured valid
    pub unsafe fn window_ptr(&self) -> *mut SDL_Window {
        self.ptr
    }

    pub fn size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            SDL_GetWindowSize(self.ptr, &mut w as *mut i32, &mut h as *mut i32);
        }
        (w as u32, h as u32)
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        unsafe {
            SDL_SetWindowSize(self.ptr, w as i32, h as i32);
        }
    }

    pub fn position(&self) -> (u32, u32) {
        let (mut x, mut y) = (0, 0);
        unsafe {
            SDL_GetWindowPosition(self.ptr, &mut x as *mut i32, &mut y as *mut i32);
        }
        (x as u32, y as u32)
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        unsafe {
            SDL_SetWindowPosition(self.ptr, x as i32, y as i32);
        }
    }

    pub fn brightness(&self) -> f32 {
        unsafe { SDL_GetWindowBrightness(self.ptr) }
    }

    pub fn set_brightness(&mut self, bright: f32) -> Result<()> {
        let err = unsafe { SDL_SetWindowBrightness(self.ptr, bright) };
        if err == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn opacity(&self) -> Result<f32> {
        let mut op = 0.0 as f32;
        if unsafe { SDL_GetWindowOpacity(self.ptr, &mut op as *mut f32) } == 0 {
            Ok(op)
        } else {
            Err(SdlError::get())
        }
    }
    pub fn set_opacity(&mut self, opacity: f32) -> Result<()> {
        let err = unsafe { SDL_SetWindowOpacity(self.ptr, opacity) };
        if err == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn title(&self) -> String {
        unsafe {
            let title = SDL_GetWindowTitle(self.ptr);
            from_sdl_string(title)
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let title = CString::new(title).unwrap();
        let ptr = title.as_ptr() as *const _;
        unsafe {
            SDL_SetWindowTitle(self.ptr, ptr);
        }
    }

    pub fn hide(&mut self) {
        unsafe {
            SDL_HideWindow(self.ptr);
        }
    }

    pub fn show(&mut self) {
        unsafe {
            SDL_ShowWindow(self.ptr);
        }
    }

    pub fn maximize(&mut self) {
        unsafe {
            SDL_MaximizeWindow(self.ptr);
        }
    }

    pub fn minimize(&mut self) {
        unsafe {
            SDL_MinimizeWindow(self.ptr);
        }
    }

    pub fn raise(&mut self) {
        unsafe {
            SDL_RaiseWindow(self.ptr);
        }
    }

    pub fn restore(&mut self) {
        unsafe { SDL_RestoreWindow(self.ptr) }
    }

    pub fn set_fullscreen(&mut self) -> Result<()> {
        let err = unsafe {
            SDL_SetWindowFullscreen(self.ptr, SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32)
        };
        if err == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn set_fullscreen_desktop(&mut self) -> Result<()> {
        let err = unsafe {
            SDL_SetWindowFullscreen(
                self.ptr,
                SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32,
            )
        };
        if err == 0 {
            Ok(())
        } else {
            Err(SdlError::get())
        }
    }

    pub fn set_resizable(&mut self) {
        unsafe {
            SDL_SetWindowResizable(self.ptr, SDL_bool::SDL_TRUE);
        }
    }

    pub fn set_unresizable(&mut self) {
        unsafe {
            SDL_SetWindowResizable(self.ptr, SDL_bool::SDL_FALSE);
        }
    }

    pub fn vk_drawable_size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            SDL_Vulkan_GetDrawableSize(self.ptr, &mut w, &mut h);
        }
        (w as _, h as _)
    }

    pub fn gl_swap(&mut self) {
        unsafe {
            SDL_GL_SwapWindow(self.ptr);
        }
    }
}
