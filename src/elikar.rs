extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::window::WindowsManager;
use crate::clipboard::Clipboard;
use crate::sysinfo::SystemInfo;

pub struct Elikar{
    windows_manager : WindowsManager,
    system_info : SystemInfo
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


impl Elikar {
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
            windows_manager : WindowsManager::new(),
            system_info : SystemInfo::new()
        })
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

}

impl Drop for Elikar {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}
