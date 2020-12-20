extern crate sdl2_sys;

use sdl2_sys::*;
use xrsignal::Signal;
use crate::mouse;

pub struct Signals {
    pub quit : Signal<(),()>,
    pub mouse_button_down : Signal<mouse::event::button::Info,()>,
    pub mouse_button_up : Signal<mouse::event::button::Info,()>,
    pub mouse_motion : Signal<mouse::event::motion::Info,()>,
    pub mouse_wheel : Signal<mouse::event::wheel::Info,()>,
}

impl Signals {
    pub fn new() -> Signals {
        Signals {
            quit : Signal::new(),
            mouse_button_down : Signal::new(),
            mouse_button_up : Signal::new(),
            mouse_motion : Signal::new(),
            mouse_wheel : Signal::new(),
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
            _ => {}
        }
    }
}