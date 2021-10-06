pub mod systems;
mod ui;
mod draw_data;

use std::cell::Ref;
pub use ui::Ui;

use xecs::System;
use std::convert::Infallible;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use imgui_sys::{igCreateContext, igDestroyContext, igGetIO, ImFontAtlas_GetGlyphRangesDefault, ImGuiContext, ImGuiIO, ImVec2};
use crate::imgui::draw_data::DrawData;
use crate::window;
use crate::window::WindowId;

pub struct ImGui {
    window_id: WindowId,
    context: *mut ImGuiContext,
    io : *mut ImGuiIO,
}

impl ImGui {
    pub fn from_window_id(window_id : WindowId) -> ImGui {
        let context = unsafe {
            igCreateContext(null_mut())
        };
        ImGui{
            window_id,
            context,
            // Safety: because context was create before
            io: unsafe { igGetIO() },
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    #[allow(dead_code)]
    pub(in crate::imgui) fn io(&self) -> &ImGuiIO {
        unsafe { &*self.io }
    }

    pub(in crate::imgui) fn io_mut(&mut self) -> &mut ImGuiIO {
        unsafe { &mut *self.io }
    }

    pub fn ui(&mut self) -> Ui<'_>{
        Ui{
            imgui: self,
            texts: vec![]
        }
    }

    pub(in crate::imgui) fn begin_frame(&mut self){
        unsafe {
            imgui_sys::igNewFrame()
        }
    }

    pub(in crate::imgui) fn render(&mut self) -> DrawData {
        unsafe { imgui_sys::igRender() };
        DrawData{
            draw_data: unsafe { imgui_sys::igGetDrawData() }
        }
    }

    pub(in crate::imgui) fn end_frame(&mut self) {
        unsafe {
            imgui_sys::igEndFrame()
        }
    }

    pub fn font_atlas(&mut self) -> &mut imgui_sys::ImFontAtlas {
        unsafe { &mut *self.io_mut().Fonts }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe { igDestroyContext(self.context) }
    }
}

impl<'a> System<'a> for ImGui {
    type InitResource = &'a window::Manager;
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;

    fn init(&'a mut self, manager : Ref<'a,window::Manager>) -> Result<(), Self::Error> {
        let window = manager.window_ref(self.window_id).unwrap();
        let (win_w,win_h) = window.size();
        let (draw_w,draw_h) = window.vk_drawable_size();

        self.io_mut().DisplaySize = ImVec2::new(win_w as _,win_h as _);
        self.io_mut().DisplayFramebufferScale = ImVec2::new(
            (draw_w as f32) / (win_w as f32),
            (draw_h as f32) / (win_h as f32));

        Ok(())
    }

    fn update(&'a mut self,_ : ()) -> Result<(), Self::Error> {
        self.begin_frame();
        Ok(())
    }

}