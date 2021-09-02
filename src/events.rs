use sdl2_sys::*;
use xecs::System;
use crate::{mouse, keyboard, drop_file};
use std::ffi::CStr;
use std::path::PathBuf;

#[derive(Default)]
pub struct PollEvents {
    pub quit : Option<()>,
    pub mouse_button_down : Option<mouse::event::button::EventInfo>,
    pub mouse_button_up : Option<mouse::event::button::EventInfo>,
    pub mouse_motion : Option<mouse::event::motion::EventInfo>,
    pub mouse_wheel : Option<mouse::event::wheel::EventInfo>,
    pub key_down : Option<keyboard::event::EventInfo>,
    pub key_up : Option<keyboard::event::EventInfo>,
    pub drop_files : Option<drop_file::event::EventInfo>,
}

impl PollEvents {
    pub fn new() -> PollEvents {
        PollEvents::default()
    }
}

impl<'a> System<'a> for PollEvents {
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self, _ : ()) {
        std::mem::swap(self,&mut PollEvents::default());

        let mut sdl_event = SDL_Event {type_:0};
        while unsafe { SDL_PollEvent(&mut sdl_event) } == 1 {
            match unsafe { sdl_event.type_ } {
                x if x == SDL_EventType::SDL_QUIT as u32 => {
                    self.quit = Some(())
                }
                x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                    self.mouse_button_down = Some(unsafe {sdl_event.button}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                    self.mouse_button_up = Some(unsafe {sdl_event.button}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                    self.mouse_motion = Some(unsafe {sdl_event.motion}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEWHEEL as u32 => {
                    self.mouse_wheel = Some(unsafe {sdl_event.wheel}.into());
                }
                x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                    self.key_down = Some(unsafe {sdl_event.key}.into());
                }
                x if x == SDL_EventType::SDL_KEYUP as u32 => {
                    self.key_up = Some(unsafe {sdl_event.key}.into());
                }
                x if x == SDL_EventType::SDL_DROPFILE as u32 => {
                    let path = PathBuf::from(unsafe{CStr::from_ptr(sdl_event.drop.file)}.to_str().unwrap());
                    unsafe{ SDL_free(sdl_event.drop.file as *mut _); }
                    self.drop_files.as_mut().unwrap().add_path(path);
                }
                x if x == SDL_EventType::SDL_DROPTEXT as u32 => {
                    // self.drop_text.emit_clone(unsafe{sdl_event.drop}.into());
                }
                x if x == SDL_EventType::SDL_DROPBEGIN as u32 => {
                    self.drop_files = Some(unsafe{sdl_event.drop}.into())
                }
                x if x == SDL_EventType::SDL_DROPCOMPLETE as u32 => {}
                _ => {}
            }
        }
    }
}