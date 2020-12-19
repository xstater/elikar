extern crate sdl2_sys;

use sdl2_sys::*;
use xrsignal::Signal;

pub struct Handlers{
    pub quit : Signal<(),()>,
    pub mouse_button_down : Signal<(i32,i32),()>,
    pub mouse_button_up : Signal<(i32,i32),()>,
    pub mouse_motion : Signal<(i32,i32),()>,

}

impl Handlers{
    pub fn new() -> Handlers{
        Handlers{
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
            },
            x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                self.mouse_button_down.emit(
                    (unsafe{sdlevent.button.x},unsafe{sdlevent.button.y}));
            },
            _ => {}
        }
    }
}