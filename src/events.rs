use sdl2_sys::*;
use xecs::System;
use crate::{mouse, keyboard, drop_file,window};
use std::ffi::CStr;
use std::path::PathBuf;

#[derive(Default)]
pub struct PollEvents {
    pub quit : Option<()>,
    pub mouse_button_down : Vec<mouse::event::button::EventInfo>,
    pub mouse_button_up : Vec<mouse::event::button::EventInfo>,
    pub mouse_motion : Vec<mouse::event::motion::EventInfo>,
    pub mouse_wheel : Vec<mouse::event::wheel::EventInfo>,
    pub key_down : Vec<keyboard::event::EventInfo>,
    pub key_up : Vec<keyboard::event::EventInfo>,
    pub drop_files : Option<drop_file::event::EventInfo>,
    pub window : Vec<window::event::EventInfo>
}

impl PollEvents {
    pub fn new() -> PollEvents {
        PollEvents::default()
    }

    fn clear(&mut self){
        self.quit = Option::None;
        self.mouse_button_down.clear();
        self.mouse_button_up.clear();
        self.mouse_motion.clear();
        self.mouse_wheel.clear();
        self.key_down.clear();
        self.key_up.clear();
        self.drop_files = Option::None;
        self.window.clear()
    }
}

impl<'a> System<'a> for PollEvents {
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self, _ : ()) {
        self.clear();

        let mut sdl_event = SDL_Event {type_:0};
        while unsafe { SDL_PollEvent(&mut sdl_event) } == 1 {
            match unsafe { sdl_event.type_ } {
                x if x == SDL_EventType::SDL_QUIT as u32 => {
                    self.quit = Some(())
                }
                x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                    self.mouse_button_down.push(unsafe {sdl_event.button}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                    self.mouse_button_up.push(unsafe {sdl_event.button}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                    self.mouse_motion.push(unsafe {sdl_event.motion}.into());
                }
                x if x == SDL_EventType::SDL_MOUSEWHEEL as u32 => {
                    self.mouse_wheel.push(unsafe {sdl_event.wheel}.into());
                }
                x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                    self.key_down.push(unsafe {sdl_event.key}.into());
                }
                x if x == SDL_EventType::SDL_KEYUP as u32 => {
                    self.key_up.push(unsafe {sdl_event.key}.into());
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
                x if x == SDL_EventType::SDL_WINDOWEVENT as u32 => {
                    self.window.push(unsafe { sdl_event.window }.into())
                }
                _ => {}
            }
        }
    }
}