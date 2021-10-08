use crate::events::PollEvents;
use crate::imgui::ImGui;
use crate::keyboard::{Code, Mod};
use crate::mouse::event::button::Button;
use imgui::{Io, Key};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use xecs::System;

pub struct ImGuiEventSystem {
    pub(in crate::imgui) mouse_pressed: [bool; 5],
}

impl ImGuiEventSystem {
    pub fn new() -> Self {
        ImGuiEventSystem {
            mouse_pressed: [false; 5],
        }
    }
}

impl<'a> System<'a> for ImGuiEventSystem {
    type InitResource = &'a mut ImGui;
    type Resource = (&'a PollEvents, &'a mut ImGui);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn init(&'a mut self, mut imgui: RefMut<'a, ImGui>) -> Result<(), Self::Error> {
        let mut io = imgui.io_mut();
        io.key_map[Key::Tab as usize] = Code::Tab as _;
        io.key_map[Key::LeftArrow as usize] = Code::Left as _;
        io.key_map[Key::RightArrow as usize] = Code::Right as _;
        io.key_map[Key::UpArrow as usize] = Code::Up as _;
        io.key_map[Key::DownArrow as usize] = Code::Down as _;
        io.key_map[Key::PageUp as usize] = Code::Pageup as _;
        io.key_map[Key::PageDown as usize] = Code::Pagedown as _;
        io.key_map[Key::Home as usize] = Code::Home as _;
        io.key_map[Key::End as usize] = Code::End as _;
        io.key_map[Key::Delete as usize] = Code::Delete as _;
        io.key_map[Key::Backspace as usize] = Code::Backspace as _;
        io.key_map[Key::Enter as usize] = Code::Return as _;
        io.key_map[Key::Escape as usize] = Code::Escape as _;
        io.key_map[Key::Space as usize] = Code::Space as _;
        io.key_map[Key::A as usize] = Code::A as _;
        io.key_map[Key::C as usize] = Code::C as _;
        io.key_map[Key::V as usize] = Code::V as _;
        io.key_map[Key::X as usize] = Code::X as _;
        io.key_map[Key::Y as usize] = Code::Y as _;
        io.key_map[Key::Z as usize] = Code::Z as _;
        Ok(())
    }

    fn update(
        &'a mut self,
        (events, mut imgui): (Ref<'a, PollEvents>, RefMut<'a, ImGui>),
    ) -> Result<(), Self::Error> {
        let io = imgui.io_mut();

        for wheel in &events.mouse_wheel {
            let (x, y) = wheel.scrolled;
            if x > 0 {
                io.mouse_wheel_h += 1.0;
            }
            if x < 0 {
                io.mouse_wheel_h -= 1.0;
            }
            if y > 0 {
                io.mouse_wheel += 1.0;
            }
            if y < 0 {
                io.mouse_wheel -= 1.0;
            }
        }
        for button in &events.mouse_button_down {
            match button.button {
                Button::Left => self.mouse_pressed[0] = true,
                Button::Middle => self.mouse_pressed[1] = true,
                Button::Right => self.mouse_pressed[2] = true,
                Button::X1 => self.mouse_pressed[3] = true,
                Button::X2 => self.mouse_pressed[4] = true,
            }
        }

        for key in &events.key_down {
            set_key_mod(io, key.mod_state);
            io.keys_down[key.code as usize] = true;
        }
        for key in &events.key_up {
            set_key_mod(io, key.mod_state);
            io.keys_down[key.code as usize] = false;
        }

        for input in &events.text_input {
            for ch in input.text.chars() {
                io.add_input_character(ch);
            }
        }

        Ok(())
    }
}

fn set_key_mod(io: &mut Io, key_mod: Mod) {
    io.key_ctrl = key_mod.ctrl();
    io.key_alt = key_mod.alt();
    io.key_shift = key_mod.shift();
    io.key_super = key_mod.gui();
}
