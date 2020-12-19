extern crate sdl2_sys;

use sdl2_sys::*;
use crate::common::get_error;
use crate::system_event::Signals;
use std::sync::{Arc,RwLock};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct ElikarBase {
    is_quit : AtomicBool,
    frames_in_second : AtomicUsize,
    frame_duration : RwLock<Duration>,
}

#[derive(Clone)]
pub struct Elikar {
    base : Arc<ElikarBase>
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
            base : Arc::new(ElikarBase{
                is_quit: AtomicBool::new(false),
                frames_in_second : AtomicUsize::new(0),
                frame_duration : RwLock::new(Duration::default()),
            })
        })
    }


    pub fn quit(&mut self) {
        self.base.is_quit.store(true,Ordering::Relaxed);
    }

    pub fn is_quit(&self) -> bool {
        self.base.is_quit.load(Ordering::Relaxed)
    }

    pub fn frame_duration(&self) -> Duration {
        *self.base.frame_duration.read().unwrap()
    }

    pub fn fps(&self) -> usize{
        1_000_000_usize.checked_div(self.frame_duration().as_micros() as usize).unwrap_or(0)
    }

    ///frames in second
    pub fn fis(&self) -> usize{
        self.base.frames_in_second.load(Ordering::Relaxed)
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
        *self.base.frame_duration.write().unwrap() = duration;
    }

    fn set_frames_in_second(&mut self, frames : usize){
        self.base.frames_in_second.store(frames,Ordering::Relaxed);
    }

}

impl Drop for ElikarBase{
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}


