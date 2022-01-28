use crate::clipboard::Clipboard;
use crate::common::SdlError;
use crate::events::Events;
use crate::events::enter_frame::EnterFrameInner;
use crate::events::leave_frame::LeaveFrameInner;
use crate::events::render::RenderInner;
use crate::events::update::UpdateInner;
use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::sysinfo::SystemInfo;
use crate::window;
use futures::Future;
use futures::executor::{LocalPool, ThreadPool};
use futures::task::{LocalSpawnExt, SpawnExt};
use sdl2_sys::*;
use xecs::world::World;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
pub use self::states::States;

mod states;

pub struct Elikar {
    local_pool : LocalPool,
    thread_pool : ThreadPool,
    events : Events,
    world : Arc<RwLock<World>>,
}

#[derive(Debug)]
pub enum SdlInitError {
    Timer(SdlError),
    Audio(SdlError),
    Video(SdlError),
    Joystick(SdlError),
    Haptic(SdlError),
    GameController(SdlError),
    Events(SdlError),
}

impl Display for SdlInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SdlInitError::Timer(err) => {
                write!(f, "Initialize SDL Time subsystem failed : {}", err.as_str())
            }
            SdlInitError::Audio(err) => {
                write!(
                    f,
                    "Initialize SDL Audio subsystem failed : {}",
                    err.as_str()
                )
            }
            SdlInitError::Video(err) => {
                write!(
                    f,
                    "Initialize SDL Video subsystem failed : {}",
                    err.as_str()
                )
            }
            SdlInitError::Joystick(err) => {
                write!(
                    f,
                    "Initialize SDL Joystick subsystem failed : {}",
                    err.as_str()
                )
            }
            SdlInitError::Haptic(err) => {
                write!(
                    f,
                    "Initialize SDL Haptic subsystem failed : {}",
                    err.as_str()
                )
            }
            SdlInitError::GameController(err) => {
                write!(
                    f,
                    "Initialize SDL GameController subsystem failed : {}",
                    err.as_str()
                )
            }
            SdlInitError::Events(err) => {
                write!(
                    f,
                    "Initialize SDL Events subsystem failed : {}",
                    err.as_str()
                )
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
            SdlInitError::Events(err) => err,
        })
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
        let mut world = World::new();

        // Pre-register some compoenents
        world.register::<window::Window>();

        // Pre-store some resource
        world.store_resource(States::new());
        world.store_resource(Mouse::new());
        world.store_resource(Keyboard::new());
        world.store_resource(Clipboard::new());
        world.store_resource(SystemInfo::new());

        let world = Arc::new(RwLock::new(world));
        let events = Events::from_world(world.clone());
        Ok(Elikar {
            local_pool: LocalPool::new(),
            thread_pool: ThreadPool::new().unwrap(),
            events,
            world,
        })
    }

    pub fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }

    pub fn events(&self) -> Events {
        self.events.clone()
    }

    pub fn window_builder(&self) -> window::Builder {
        window::Builder::from_world(self.world.clone())
    }

    pub fn spawn_local<F>(&mut self,f : F)
    where F : Future<Output = ()> + 'static{
        self.local_pool.spawner().spawn_local(f).unwrap();
    }

    pub fn spawn<F>(&mut self,f : F)
    where F : Future<Output = ()> + Send + 'static {
        self.thread_pool.spawn(f).unwrap();
    }

    pub fn run(mut self) {
        let events = self.events();
        let mut frame = 1;
        'mainloop : loop {
            let start_time = Instant::now();
            // Quit checking
            {
                let world = self.world.read().unwrap();
                if world.resource_ref::<States>()
                    .expect("Elikar run(): Game states was moved unexpectly")
                    .quit {
                    break 'mainloop;
                }
            }
            // enter frame
            {
                let world = self.world.read().unwrap();
                for inner in world.query::<&EnterFrameInner>() {
                    inner.tx.send(frame).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // events poll
            events.poll();
            self.local_pool.run_until_stalled();
            // Update
            {
                let world = self.world.read().unwrap();
                for inner in world.query::<&UpdateInner>() {
                    inner.tx.send(()).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // Render
            {
                let world = self.world.read().unwrap();
                for inner in world.query::<&RenderInner>() {
                    inner.tx.send(()).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // leave frame
            {
                let world = self.world.read().unwrap();
                for inner in world.query::<&LeaveFrameInner>() {
                    inner.tx.send(frame).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // update elikar state
            {
                let world = self.world.read().unwrap();
                let mut states = world.resource_mut::<States>().unwrap();
                states.frame_counter += 1;
                if states.sec_timer.elapsed() > Duration::from_secs(1) {
                    states.frames_in_sec = states.frame_counter;
                    states.frame_counter = 0;
                    states.sec_timer = Instant::now();
                }
                states.last_frame_time = start_time.elapsed();
            }
            frame += 1;
        }
    }
}

impl Drop for Elikar {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

