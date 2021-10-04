use xecs::System;
use crate::imgui::ImGui;
use crate::keyboard::Code;
use std::cell::RefMut;
use std::convert::Infallible;
use crate::events::PollEvents;

pub struct ImGuiEventSystem {
}

impl ImGuiEventSystem {
    pub fn new() -> Self {
        ImGuiEventSystem{
        }
    }
}

impl<'a> System<'a> for ImGuiEventSystem {
    type InitResource = &'a mut ImGui;
    type Resource = &'a mut ImGui;
    type Dependencies = (PollEvents,ImGui);
    type Error = Infallible;

    fn init(&'a mut self, mut imgui: RefMut<'a,ImGui>) -> Result<(), Self::Error> {
        let mut io = imgui.io_mut();
        io.KeyMap[imgui_sys::ImGuiKey_Tab as usize] = Code::Tab as _;
        io.KeyMap[imgui_sys::ImGuiKey_LeftArrow as usize] = Code::Left as _;
        io.KeyMap[imgui_sys::ImGuiKey_RightArrow as usize] = Code::Right as _;
        io.KeyMap[imgui_sys::ImGuiKey_UpArrow as usize] = Code::Up as _;
        io.KeyMap[imgui_sys::ImGuiKey_DownArrow as usize] = Code::Down as _;
        io.KeyMap[imgui_sys::ImGuiKey_PageUp as usize] = Code::Pageup as _;
        io.KeyMap[imgui_sys::ImGuiKey_PageDown as usize] = Code::Pagedown as _;
        io.KeyMap[imgui_sys::ImGuiKey_Home as usize] = Code::Home as _;
        io.KeyMap[imgui_sys::ImGuiKey_End as usize] = Code::End as _;
        io.KeyMap[imgui_sys::ImGuiKey_Delete as usize] = Code::Delete as _;
        io.KeyMap[imgui_sys::ImGuiKey_Backspace as usize] = Code::Backspace as _;
        io.KeyMap[imgui_sys::ImGuiKey_Enter as usize] = Code::Return as _;
        io.KeyMap[imgui_sys::ImGuiKey_Escape as usize] = Code::Escape as _;
        io.KeyMap[imgui_sys::ImGuiKey_Space as usize] = Code::Space as _;
        io.KeyMap[imgui_sys::ImGuiKey_A as usize] = Code::A as _;
        io.KeyMap[imgui_sys::ImGuiKey_C as usize] = Code::C as _;
        io.KeyMap[imgui_sys::ImGuiKey_V as usize] = Code::V as _;
        io.KeyMap[imgui_sys::ImGuiKey_X as usize] = Code::X as _;
        io.KeyMap[imgui_sys::ImGuiKey_Y as usize] = Code::Y as _;
        io.KeyMap[imgui_sys::ImGuiKey_Z as usize] = Code::Z as _;
        Ok(())
    }

    fn update(&'a mut self,_imgui : RefMut<'a,ImGui>) -> Result<(), Self::Error> {
        Ok(())
    }
}

