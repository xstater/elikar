extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::window::WindowsManager;
use crate::clipboard::Clipboard;
use crate::sysinfo::SystemInfo;
use crate::mouse::Mouse;
use crate::event::Event;

pub struct Elikar<'a>{
    is_quit : bool,
    windows_manager : WindowsManager,
    system_info : SystemInfo,
    mouse : Mouse<'a>
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


impl<'a> Elikar<'a>{
    pub fn new() -> Result<Elikar<'a>,SdlInitError> {
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
            windows_manager : WindowsManager::new(),
            system_info : SystemInfo::new(),
            mouse : Mouse{
                on_button_down : Event::new(),
                on_button_up : Event::new(),
                on_motion : Event::new()
            }
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

    pub fn quit(&mut self){
        self.is_quit = true;
    }

    pub fn clipboard(&self) -> Clipboard{
        Clipboard::new()
    }

    pub fn windows_manager(&self) -> &WindowsManager{
        &self.windows_manager
    }

    pub fn windows_manager_mut(&mut self) -> &mut WindowsManager{
        &mut self.windows_manager
    }

    pub fn system_info(&self) -> &SystemInfo{
        &self.system_info
    }

    pub fn mouse(&self) -> &Mouse<'a> {
        &self.mouse
    }

    pub fn mouse_mut(&mut self) -> &mut Mouse<'a> {
        &mut self.mouse
    }

}

impl<'a> Drop for Elikar<'a>{
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}
