extern crate sdl2_sys;

use sdl2_sys::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::common::SdlError;
use xecs::{System, Stage};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use crate::window;
use crate::clipboard::Clipboard;
use crate::sysinfo::SystemInfo;
use std::any::TypeId;

#[derive(Debug)]
pub struct ElikarStates {
    quit : bool,
    frames_count : usize,
    start_time : Instant,
    frame_time : Duration,
    add_stage_buffer : Option<Vec<(String,Stage)>>,
    remove_stage_buffer : Option<Vec<String>>,
    change_current : Option<String>,
    deactivated_system_buffer : Option<Vec<TypeId>>,
    activated_system_buffer : Option<Vec<TypeId>>
}

impl<'a> System<'a> for ElikarStates {
    type Resource = ();
    type Dependencies = ();
}

#[derive(Debug)]
pub struct Elikar {
    current_stage : String,
    stages : HashMap<String,Stage>
}

#[derive(Debug)]
pub enum SdlInitError{
    Timer(SdlError),
    Audio(SdlError),
    Video(SdlError),
    Joystick(SdlError),
    Haptic(SdlError),
    GameController(SdlError),
    Events(SdlError)
}

impl Display for SdlInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SdlInitError::Timer(err) => {
                write!(f,"Initialize SDL Time subsystem failed : {}", err.as_str())
            }
            SdlInitError::Audio(err) => {
                write!(f,"Initialize SDL Audio subsystem failed : {}", err.as_str())
            }
            SdlInitError::Video(err) => {
                write!(f,"Initialize SDL Video subsystem failed : {}", err.as_str())
            }
            SdlInitError::Joystick(err) => {
                write!(f,"Initialize SDL Joystick subsystem failed : {}", err.as_str())
            }
            SdlInitError::Haptic(err) => {
                write!(f,"Initialize SDL Haptic subsystem failed : {}", err.as_str())
            }
            SdlInitError::GameController(err) => {
                write!(f,"Initialize SDL GameController subsystem failed : {}", err.as_str())
            }
            SdlInitError::Events(err) => {
                write!(f,"Initialize SDL Events subsystem failed : {}", err.as_str())
            }
        }
    }
}

impl Error for SdlInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match &self {
            SdlInitError::Timer(err) => err,
            SdlInitError::Audio(err) => err,
            SdlInitError::Video(err) => err,
            SdlInitError::Joystick(err) => err,
            SdlInitError::Haptic(err) => err,
            SdlInitError::GameController(err) => err,
            SdlInitError::Events(err) => err
        })
    }
}

#[derive(Debug,Default,Clone)]
pub struct StageNotFound(String);

impl Display for StageNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Cannot find any stage named {}",&self.0)
    }
}

impl Error for StageNotFound{}

#[derive(Debug,Clone)]
pub enum RemoveStageError {
    UsingNow,
    StageNotFound(StageNotFound)
}

impl Display for RemoveStageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveStageError::UsingNow => {
                write!(f,"Stage is current stage so that cannot be removed.")
            }
            RemoveStageError::StageNotFound(stage_not_found) => {
                stage_not_found.fmt(f)
            }
        }
    }
}

impl Error for RemoveStageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RemoveStageError::UsingNow => Option::None,
            RemoveStageError::StageNotFound(stage_not_found) => Some(stage_not_found)
        }
    }
}

impl Elikar {
    pub fn new() -> Result<Elikar, SdlInitError> {
        // if unsafe { SDL_InitSubSystem(SDL_INIT_TIMER) } != 0 {
        //     return Err(SdlInitError::Timer(get_error()));
        // }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) } != 0 {
        //     return Err(SdlInitError::Audio(get_error()));
        // }
        if unsafe { SDL_InitSubSystem(SDL_INIT_VIDEO) } != 0 {
            return Err(SdlInitError::Video(SdlError::get()));
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
            return Err(SdlInitError::Events(SdlError::get()));
        }
        let mut stage = Stage::new();
        stage.add_system(ElikarStates {
            quit: false,
            frames_count : 0,
            start_time : Instant::now(),
            frame_time: Duration::from_secs(1),
            add_stage_buffer: Option::None,
            remove_stage_buffer: Option::None,
            change_current: Option::None,
            deactivated_system_buffer: Option::None,
            activated_system_buffer: Option::None
        });
        stage.deactivate::<ElikarStates>();
        Ok(Elikar {
            current_stage: "default".to_owned(),
            stages: {
                let mut map = HashMap::new();
                map.insert("default".to_owned(),stage);
                map
            }
        })
    }

    pub fn create_window_manager(&self) -> window::Manager {
        window::Manager::new()
    }

    pub fn clipboard(&self) -> Clipboard {
        Clipboard
    }

    pub fn system_info(&self) -> SystemInfo {
        SystemInfo
    }

    pub fn current_stage_ref(&self) -> &Stage {
        // elikar will ensure the current stage existence.
        self.stages.get(&self.current_stage).unwrap()
    }

    pub fn current_stage_mut(&mut self) -> &mut Stage {
        // elikar will ensure the current stage existence.
        self.stages.get_mut(&self.current_stage).unwrap()
    }

    pub fn current_stage_name(&self) -> &str {
        self.current_stage.as_str()
    }

    pub fn stage_names(&self) -> Vec<String> {
        self.stages.keys()
            .cloned()
            .collect()
    }

    pub fn has_stage(&self,name : &str) -> bool {
        self.stages.contains_key(name)
    }

    pub fn change_current(&mut self,name : &str) -> Result<(),StageNotFound> {
        if self.has_stage(name) {
            self.current_stage = name.to_owned();
            Ok(())
        } else {
            Err(StageNotFound(name.to_owned()))
        }
    }

    pub fn remove_stage(&mut self,name : &str) -> Result<Stage,StageNotFound> {
        self.stages.remove(name).ok_or(StageNotFound(name.to_owned()))
    }

    pub fn add_stage(&mut self,name : &str,mut stage : Stage) {
        stage.add_system(ElikarStates{
            quit: false,
            frames_count: 0,
            start_time: Instant::now(),
            frame_time: Duration::from_secs(2),
            add_stage_buffer: Option::None,
            remove_stage_buffer: Option::None,
            change_current: Option::None,
            deactivated_system_buffer: Option::None,
            activated_system_buffer: Option::None
        });
        self.stages.insert(name.to_owned(), stage);
    }

    pub fn run(&mut self) {
        let mut frames_count = 0;
        'main_loop: loop {
            let now = Instant::now();
            {
                let stage = self.current_stage_ref();
                let mut states = stage.system_data_mut::<ElikarStates>();
                if states.quit {
                    break 'main_loop;
                }
                if states.start_time.elapsed().as_secs() > 0 {
                    states.frames_count = frames_count;
                    frames_count = 0;
                    states.start_time = Instant::now()
                }
            }
            // run stage
            self.current_stage_mut().run();
            // after run
            // activate systems
            let systems = self.current_stage_ref()
                .system_data_mut::<ElikarStates>()
                .activated_system_buffer
                .take();
            if let Some(systems) = systems {
                for system in systems {
                    self.current_stage_mut()
                        .activate_dyn(system);
                }
            }
            // deactivate systems
            let systems = self.current_stage_ref()
                .system_data_mut::<ElikarStates>()
                .deactivated_system_buffer
                .take();
            if let Some(systems) = systems {
                for system in systems {
                    self.current_stage_mut()
                        .deactivate_dyn(system);
                }
            }
            // add stage
            let add_buffer = self.current_stage_ref()
                    .system_data_mut::<ElikarStates>()
                    .add_stage_buffer
                    .take();
            if let Some(buffer) = add_buffer {
                for (name,stage) in buffer {
                    self.add_stage(name.as_str(),stage);
                }
            }
            // remove stage
            let remove_buffer = self.current_stage_ref()
                .system_data_mut::<ElikarStates>()
                .remove_stage_buffer
                .take();
            if let Some(buffer) = remove_buffer {
                for name in buffer {
                    self.remove_stage(name.as_str()).unwrap();
                }
            }
            // change current stage
            let current = self.current_stage_ref()
                .system_data_mut::<ElikarStates>()
                .change_current
                .take();
            if let Some(name) = current {
                self.change_current(name.as_str()).unwrap();
            }
            frames_count += 1;
            self.current_stage_ref()
                .system_data_mut::<ElikarStates>().frame_time = now.elapsed();
        }
    }
}

impl Drop for Elikar{
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

impl ElikarStates {
    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn actual_fps(&self) -> usize {
        self.frames_count
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.frame_time.as_secs_f64()
    }

    pub fn add_stage(&mut self,name : &str,stage : Stage) {
        if self.add_stage_buffer.is_none() {
            self.add_stage_buffer = Some(vec![]);
        }
        self.add_stage_buffer
            .as_mut()
            .unwrap()
            .push((name.to_owned(),stage));
    }

    pub fn remove_stage(&mut self,name : &str) {
        if self.remove_stage_buffer.is_none() {
            self.remove_stage_buffer = Some(vec![]);
        }
        self.remove_stage_buffer
            .as_mut()
            .unwrap()
            .push(name.to_owned());
    }

    pub fn change_current(&mut self,name : &str) {
        self.change_current = Some(name.to_owned());
    }

    pub fn activate_system<T : for<'a> System<'a>>(&mut self){
        if self.activated_system_buffer.is_none() {
            self.activated_system_buffer = Some(vec![]);
        }
        self.activated_system_buffer
            .as_mut()
            .unwrap()
            .push(TypeId::of::<T>());
    }

    pub fn deactivate_system<T : for<'a> System<'a>>(&mut self) {
        if self.deactivated_system_buffer.is_none() {
            self.deactivated_system_buffer = Some(vec![]);
        }
        self.deactivated_system_buffer
            .as_mut()
            .unwrap()
            .push(TypeId::of::<T>());
    }
}
