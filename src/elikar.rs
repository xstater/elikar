extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::system_event::Signals;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};

pub struct ElikarBase {
    is_quit : bool,
    frames_in_second : usize,
    frame_duration : Duration,
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
                frames_in_second : 0,
                frame_duration : Duration::default(),
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

    pub fn frame_duration(&self) -> Duration {
        if let Ok(guard) = self.base.lock() {
            guard.borrow().frame_duration
        }else{
            Duration::default()
        }
    }

    pub fn fps(&self) -> usize{
        1_000_000 / self.frame_duration().as_micros() as usize
    }

    ///frames in second
    pub fn fis(&self) -> usize{
        if let Ok(guard) = self.base.lock() {
            guard.borrow().frames_in_second
        }else{
            0
        }
    }

    pub fn run(&mut self, mut event_handlers : Signals){
        let mut sdlevent : SDL_Event = SDL_Event{type_ : 0};
        let mut frames : usize = 0;
        let mut second_time = Instant::now();
        while !self.is_quit() {
            let frame_start_time = Instant::now();
            if second_time.elapsed() > Duration::from_secs(1){
                self.set_frames_in_second(frames);
                frames = 0;
                second_time = Instant::now()
            }
            while unsafe{ SDL_PollEvent(&mut sdlevent) } == 1 {
                event_handlers.dispatch(sdlevent);
            }
            self.set_frame_duration(frame_start_time.elapsed());
            frames += 1;
        }
    }

    fn set_frame_duration(&mut self,duration: Duration){
        if let Ok(guard) = self.base.lock() {
            guard.borrow_mut().frame_duration = duration;
        }
    }

    fn set_frames_in_second(&mut self, frames : usize){
        if let Ok(guard) = self.base.lock() {
            guard.borrow_mut().frames_in_second = frames;
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


