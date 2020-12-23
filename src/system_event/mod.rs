extern crate sdl2_sys;

use sdl2_sys::*;
use xrsignal::Signal;
use crate::{mouse, keyboard,drop_file};
use std::ffi::CStr;
use std::path::PathBuf;

pub struct Signals {
    pub quit : Signal<(),()>,
    pub enter_frame : Signal<(),()>,
    pub leave_frame : Signal<(),()>,
    pub mouse_button_down : Signal<mouse::event::button::Info,()>,
    pub mouse_button_up : Signal<mouse::event::button::Info,()>,
    pub mouse_motion : Signal<mouse::event::motion::Info,()>,
    pub mouse_wheel : Signal<mouse::event::wheel::Info,()>,
    pub key_down : Signal<keyboard::event::Info,()>,
    pub key_up : Signal<keyboard::event::Info,()>,
    pub drop_files : Signal<drop_file::event::Info,()>,
    drop_files_info : Option<drop_file::event::Info>
}

impl Signals {
    pub fn new() -> Signals {
        Signals {
            quit : Signal::new(),
            enter_frame : Signal::new(),
            leave_frame : Signal::new(),
            mouse_button_down : Signal::new(),
            mouse_button_up : Signal::new(),
            mouse_motion : Signal::new(),
            mouse_wheel : Signal::new(),
            key_down : Signal::new(),
            key_up : Signal::new(),
            drop_files : Signal::new(),
            drop_files_info : Option::None,
        }
    }

    pub(in crate) fn dispatch(&mut self,sdl_event: SDL_Event){
        match unsafe { sdl_event.type_ } {
            x if x == SDL_EventType::SDL_QUIT as u32 => {
                self.quit.emit(());
            }
            x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                self.mouse_button_down.emit(unsafe{sdl_event.button}.into());
            }
            x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                self.mouse_button_up.emit(unsafe{sdl_event.button}.into());
            }
            x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                self.mouse_motion.emit(unsafe{sdl_event.motion}.into());
            }
            x if x == SDL_EventType::SDL_MOUSEWHEEL as u32 => {
                self.mouse_wheel.emit(unsafe{sdl_event.wheel}.into());
            }
            x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                self.key_down.emit(unsafe{sdl_event.key}.into());
            }
            x if x == SDL_EventType::SDL_KEYUP as u32 => {
                self.key_up.emit(unsafe{sdl_event.key}.into());
            }
            x if x == SDL_EventType::SDL_DROPFILE as u32 => {
                let path = PathBuf::from(unsafe{CStr::from_ptr(sdl_event.drop.file)}.to_str().unwrap());
                unsafe{ SDL_free(sdl_event.drop.file as *mut _); }
                self.drop_files_info.as_mut().unwrap().add_path(path)
            }
            x if x == SDL_EventType::SDL_DROPTEXT as u32 => {
                // self.drop_text.emit_clone(unsafe{sdl_event.drop}.into());
            }
            x if x == SDL_EventType::SDL_DROPBEGIN as u32 => {
                self.drop_files_info = Some(unsafe{sdl_event.drop}.into())
            }
            x if x == SDL_EventType::SDL_DROPCOMPLETE as u32 => {
                //no possibility to failed
                self.drop_files.emit_clone(self.drop_files_info.take().unwrap());
            }
            _ => {}
        }
    }
}