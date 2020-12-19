extern crate sdl2_sys;

use sdl2_sys::*;
use xrsignal::Signal;
use crate::mouse;

pub struct Signals {
    pub quit : Signal<(),()>,
    pub mouse_button_down : Signal<mouse::button::event::Info,()>,
    pub mouse_button_up : Signal<mouse::button::event::Info,()>,
    pub mouse_motion : Signal<(i32,i32),()>,

}

impl Signals {
    pub fn new() -> Signals {
        Signals {
            quit : Signal::new(),
            mouse_button_down : Signal::new(),
            mouse_button_up : Signal::new(),
            mouse_motion : Signal::new(),
        }
    }

    pub(in crate) fn dispatch(&mut self,sdlevent : SDL_Event){
        match unsafe { sdlevent.type_ } {
            x if x == SDL_EventType::SDL_QUIT as u32 => {
                self.quit.emit(());
            }
            x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                self.mouse_button_down.emit(unsafe{sdlevent.button}.into());
            }
            x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                self.mouse_button_up.emit(unsafe{sdlevent.button}.into());
            }
            x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                self.mouse_motion.emit(
                    (unsafe{sdlevent.button.x},unsafe{sdlevent.button.y}));
            }
            _ => {}
        }
    }
}