extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::sysinfo::SystemInfo;
use crate::window;
use crate::clipboard::Clipboard;

pub struct Elikar{
    is_quit : bool,
    windows_manager : window::Manager,
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
            is_quit : false,
            windows_manager : window::Manager::_new(),
        })
    }

    pub fn run(&mut self){
        let mut sdlevent : SDL_Event = SDL_Event{type_ : 0};
        while !self.is_quit {
            while unsafe{ SDL_PollEvent(&mut sdlevent) } == 1 {
                match unsafe { sdlevent.type_ } {
                    x if x == SDL_EventType::SDL_QUIT as u32 => {
                        self.is_quit = true;
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn quit(&mut self) {
        self.is_quit = true;
    }

    pub fn system_info(&self) -> SystemInfo{
        SystemInfo::_new()
    }

    pub fn windows_manager_ref(&self) -> &window::Manager{
        &self.windows_manager
    }

    pub fn windows_manager_mut(&mut self) -> &mut window::Manager {
        &mut self.windows_manager
    }

    pub fn clipboard(&self) -> Clipboard{
        Clipboard::_new()
    }
}

impl Drop for Elikar{
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}
