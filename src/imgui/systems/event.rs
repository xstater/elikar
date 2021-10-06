use xecs::System;
use crate::imgui::ImGui;
use crate::keyboard::{Code, Mod};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use crate::events::PollEvents;
use crate::mouse::event::button::Button;

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
    type Resource = (&'a PollEvents,&'a mut ImGui);
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

    fn update(&'a mut self,(events,mut imgui) : (Ref<'a,PollEvents>,RefMut<'a,ImGui>)) -> Result<(), Self::Error> {
        let io = imgui.io_mut();
        for wheel in &events.mouse_wheel {
            io.MouseWheel = wheel.scrolled.1 as _;
        }
        for button in &events.mouse_button_down {
            io.MouseDown[get_mouse_button_index(button.button)] = true;
        }
        for button in &events.mouse_button_up {
            io.MouseDown[get_mouse_button_index(button.button)] = false;
        }
        for motion in &events.mouse_motion {
            io.MousePos = imgui_sys::ImVec2::new(motion.position.0 as _,motion.position.1 as _);
        }

        for key in &events.key_down {
            set_key_mod(io,key.mod_state);
            io.KeysDown[key.code as usize] = true;
        }
        for key in &events.key_up {
            set_key_mod(io,key.mod_state);
            io.KeysDown[key.code as usize] = false;
        }
        Ok(())
    }
}

fn get_mouse_button_index(button : Button) -> usize{
    match button {
        Button::Left => 0,
        Button::Middle => 2,
        Button::Right => 1,
        Button::X1 => 3,
        Button::X2 => 4
    }
}

fn set_key_mod(io : &mut imgui_sys::ImGuiIO,key_mod : Mod){
    io.KeyCtrl = key_mod.ctrl();
    io.KeyAlt = key_mod.alt();
    io.KeyShift = key_mod.shift();
    io.KeySuper = key_mod.gui();
}