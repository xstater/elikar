extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::system_event::Handlers;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};

pub struct ElikarBase {
    is_quit : bool,
}

#[derive(Clone)]
pub struct Elikar {
    base : Arc<Mutex<RefCell<ElikarBase>>>
}

#[derive(Debug)]
pub enum SdlInitError{
    Timer(String),
    Audio(String),
    Video(String),
    Joystick(String),
    Haptic(String),
    GameController(String),
    Events(String)
}


impl Elikar{
    pub fn new() -> Result<Elikar,SdlInitError> {
        // if unsafe { SDL_InitSubSystem(SDL_INIT_TIMER) } != 0 {
        //     return Err(SdlInitError::Timer(get_error()));
        // }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) } != 0 {
        //     return Err(SdlInitError::Audio(get_error()));
        // }
        if unsafe { SDL_InitSubSystem(SDL_INIT_VIDEO) } != 0 {
            return Err(SdlInitError::Video(get_error()));
        }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_JOYSTICK) } != 0 {
        //     return Err(SdlInitError::Joystick(get_error()));
        // }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_HAPTIC) } != 0 {
        //     return Err(SdlInitError::Haptic(get_error()));
        // }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_GAMECONTROLLER) } != 0 {
        //     return Err(SdlInitError::GameController(get_error()));
        // }
        if unsafe { SDL_InitSubSystem(SDL_INIT_EVENTS) } != 0 {
            return Err(SdlInitError::Events(get_error()));
        }
        Ok(Elikar{
            base : Arc::new(Mutex::new(RefCell::new(ElikarBase{
                is_quit: false,
            })))
        })
    }


    pub fn quit(&mut self) {
        if let Ok(guard) = self.base.lock(){
            let mut base = guard.borrow_mut();
            base.is_quit = true;
        }
    }

    pub fn is_quit(&self) -> bool {
        if let Ok(guard) = self.base.lock() {
            guard.borrow().is_quit
        }else{
            false
        }
    }

    pub fn run(&mut self, mut event_handlers : Handlers){
        let mut sdlevent : SDL_Event = SDL_Event{type_ : 0};
        while !self.is_quit() {
            while unsafe{ SDL_PollEvent(&mut sdlevent) } == 1 {
                event_handlers.dispatch(sdlevent);
            }
        }
    }
}

impl Drop for ElikarBase{
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}


