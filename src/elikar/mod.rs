use crate::clipboard::Clipboard;
use crate::common::{Handle, SdlError, Spawner};
use crate::events::Events;
use crate::events::enter_frame::EnterFrameInner;
use crate::events::leave_frame::LeaveFrameInner;
use crate::events::render::RenderInner;
use crate::events::update::UpdateInner;
use crate::ime::IME;
use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::sysinfo::SystemInfo;
use crate::{quit, time, window};
use futures::{Future, FutureExt};
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use log::{error, info, trace};
use parking_lot::RwLock;
use sdl2_sys::*;
use tokio::runtime::Runtime;
use tokio_util::context::TokioContext;
use xecs::world::World;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

mod world;

pub use world::ElikarWorld;

pub struct Elikar {
    local_pool : LocalPool,
    tokio_runtime : Runtime,
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
    pub(in crate) fn new() -> Result<Elikar, SdlInitError> {
        info!(target: "Elikar","Initializing Elikar");
        // if unsafe { SDL_InitSubSystem(SDL_INIT_TIMER) } != 0 {
        //     return Err(SdlInitError::Timer(get_error()));
        // }
        // if unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) } != 0 {
        //     return Err(SdlInitError::Audio(get_error()));
        // }
        trace!(target: "Elikar","Initializing SDL video subsystem");
        if unsafe { SDL_InitSubSystem(SDL_INIT_VIDEO) } != 0 {
            error!(target: "Elikar","Initialize SDL video subsystem failed");
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
        trace!(target: "Elikar","Initializing SDL events subsystem");
        if unsafe { SDL_InitSubSystem(SDL_INIT_EVENTS) } != 0 {
            error!(target: "Elikar","Initialize SDL events subsystem failed");
            return Err(SdlInitError::Events(SdlError::get()));
        }

        let mut world = World::new();

        // Pre-register some compoenents
        trace!(target: "Elikar","Register elikar compoenents");
        world.register::<window::Window>();

        // Pre-store some resource
        trace!(target: "Elikar","Register elikar resources");
        world.register_resource(Mouse::new());
        world.register_resource(Keyboard::new());
        world.register_resource(Clipboard::new());
        world.register_resource(SystemInfo::new());
        world.register_resource(IME::new());
        world.register_resource(quit::Quit::new());
        world.register_resource(time::Time::new());

        let world = Arc::new(RwLock::new(world));
        let events = Events::from_world(world.clone());
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Ok(Elikar {
            local_pool: LocalPool::new(),
            tokio_runtime: runtime,
            events,
            world,
        })
    }

    pub fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }

    pub fn elikar_world(&self) -> ElikarWorld<'_> {
        ElikarWorld::new(self.world.read())
    }

    pub fn events(&self) -> Events {
        self.events.clone()
    }

    pub fn window_builder(&self) -> window::Builder {
        window::Builder::from_world(self.world.clone())
    }

    pub fn run(mut self) {
        info!(target: "Elikar","Start running");
        let events = self.events();
        // as least run all future once to register their waker to World
        self.local_pool.run_until_stalled();
        'mainloop : loop {
            // Quit checking & Get frame counter
            let frame = {
                let world = self.elikar_world();
                if world.need_quit() {
                    break 'mainloop;
                }
                let frame = world.time().frame_counter() as u32;
                frame
            };
            // enter frame
            {
                let world = self.world.read();
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
                let world = self.world.read();
                for inner in world.query::<&UpdateInner>() {
                    inner.tx.send(()).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // Render
            {
                let world = self.world.read();
                for inner in world.query::<&RenderInner>() {
                    inner.tx.send(()).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // leave frame
            {
                let world = self.world.read();
                for inner in world.query::<&LeaveFrameInner>() {
                    inner.tx.send(frame).unwrap();
                    inner.waker.wake_by_ref();
                }
            }
            self.local_pool.run_until_stalled();
            // update elikar state
            {
                let world = self.elikar_world();
                world.time_mut().tick();
            }
        }
    }
}

impl Spawner for Elikar {
    fn spawn<F>(&mut self,f : F) -> Handle
    where F : Future<Output = ()> + Send + 'static {
        info!(target: "Elikar","Spawn an async task");
        let handle = self.tokio_runtime.spawn(f);
        Handle::tokio(handle)
    }

    fn spawn_local<F>(&mut self,f : F) -> Handle
    where F : Future<Output = ()> + 'static {
        info!(target: "Elikar","Spawn a local task");
        let (f,abort_handle) = futures::future::abortable(f);
        let rt_handle = self.tokio_runtime.handle().clone();
        let task_handle = self.local_pool.spawner()
            .spawn_local_with_handle(TokioContext::new(f.map(|_|()),rt_handle))
            .unwrap();
        Handle::futures(abort_handle, task_handle)
    }
}

impl Drop for Elikar {
    fn drop(&mut self) {
        unsafe {
            info!(target:"Elikar","Quit Elikar");
            SDL_Quit();
        }
    }
}

