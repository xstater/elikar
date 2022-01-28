use std::sync::{Arc, RwLock};
use crate::{drop_file, keyboard, mouse, window};
use xecs::world::World;
use self::quit::QuitEvent;
use sdl2_sys::*;

mod quit;
pub(in crate) mod enter_frame;
pub(in crate) mod leave_frame;
pub(in crate) mod update;
pub(in crate) mod render;

pub use quit::Quit;
pub use enter_frame::EnterFrame;
pub use leave_frame::LeaveFrame;
pub use update::Update;
pub use render::Render;

#[derive(Clone)]
pub struct Events {
    world : Arc<RwLock<World>>
}

impl Events {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Events {
        {
            let mut world = world.write().unwrap();
            world.register::<QuitEvent>()
                .register::<mouse::event::button::ButtonDownInner>()
                .register::<mouse::event::button::ButtonUpInner>()
                .register::<mouse::event::motion::MotionInner>()
                .register::<mouse::event::wheel::WheelInner>()
                .register::<window::event::WindowEventInner>()
                .register::<keyboard::event::KeyDownInner>()
                .register::<keyboard::event::KeyUpInner>()
                .register::<drop_file::event::DropEventInner>()
                .register::<enter_frame::EnterFrameInner>()
                .register::<leave_frame::LeaveFrameInner>()
                .register::<update::UpdateInner>()
                .register::<render::RenderInner>();
        }
        Events {
            world
        }
    }

    pub(in crate) fn poll(&self) {
        let mut sdl_event = SDL_Event{ type_ : 0 };
        while unsafe { SDL_PollEvent(&mut sdl_event) } == 1 {
            let world = self.world.read().unwrap();
            match unsafe { sdl_event.type_ } {
                x if x == SDL_EventType::SDL_QUIT as u32 => {
                    for quit_event in world.query::<&QuitEvent>() {
                        // This will never fails
                        quit_event.tx.send(()).unwrap();
                        quit_event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                    let event_info = mouse::event::button::EventInfo::from_sdl_event(&world,unsafe { sdl_event.button });
                    for event in world.query::<&mouse::event::button::ButtonDownInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                    let event_info = mouse::event::button::EventInfo::from_sdl_event(&world,unsafe { sdl_event.button });
                    for event in world.query::<&mouse::event::button::ButtonUpInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                    let event_info = mouse::event::motion::EventInfo::from_sdl_event(&world,unsafe { sdl_event.motion });
                    for event in world.query::<&mouse::event::motion::MotionInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEWHEEL as u32 => {
                    let event_info = mouse::event::wheel::EventInfo::from_sdl_event(&world, unsafe { sdl_event.wheel });
                    for event in world.query::<&mouse::event::wheel::WheelInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                    let event_info = keyboard::event::EventInfo::from_sdl_event(&world, unsafe { sdl_event.key });
                    for event in world.query::<&keyboard::event::KeyDownInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_KEYUP as u32 => {
                    let event_info = keyboard::event::EventInfo::from_sdl_event(&world, unsafe { sdl_event.key });
                    for event in world.query::<&keyboard::event::KeyUpInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_WINDOWEVENT as u32 => {
                    let event_info = window::event::EventInfo::from_sdl_event(&world, unsafe { sdl_event.window });
                    for event in world.query::<&window::event::WindowEventInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_DROPBEGIN as u32 => {
                },
                x if x == SDL_EventType::SDL_DROPFILE as u32 => {
                    let event_info = drop_file::event::EventInfo::from_sdl_event(&world,unsafe { sdl_event.drop });
                    for event in world.query::<&drop_file::event::DropEventInner>() {
                        event.tx.send(event_info.clone()).unwrap();
                        event.waker.wake_by_ref()
                    }
                },
                x if x == SDL_EventType::SDL_DROPCOMPLETE as u32 => {
                },
                _ => {}
            }
        }
    }

    pub fn on_quit(&self) -> Quit {
        Quit::from_world(self.world.clone())
    }

    pub fn on_mouse_down(&self) -> mouse::event::button::ButtonDown {
        mouse::event::button::ButtonDown::from_world(self.world.clone())
    }

    pub fn on_mouse_up(&self) -> mouse::event::button::ButtonUp{
        mouse::event::button::ButtonUp::from_world(self.world.clone())
    }

    pub fn on_mouse_motion(&self) -> mouse::event::motion::Motion {
        mouse::event::motion::Motion::from_world(self.world.clone())
    }

    pub fn on_mouse_wheel(&self) -> mouse::event::wheel::Wheel {
        mouse::event::wheel::Wheel::from_world(self.world.clone())
    }

    pub fn on_key_down(&self) -> keyboard::event::KeyDown {
        keyboard::event::KeyDown::from_world(self.world.clone())
    }

    pub fn on_key_up(&self) -> keyboard::event::KeyUp {
        keyboard::event::KeyUp::from_world(self.world.clone())
    }

    pub fn on_window_events(&self) -> window::event::WindowEvent {
        window::event::WindowEvent::from_world(self.world.clone())
    }

    pub fn on_drop_file(&self) -> drop_file::event::DropEvent {
        drop_file::event::DropEvent::from_world(self.world.clone())
    }


    pub fn on_enter_frame(&self) -> EnterFrame {
        EnterFrame::from_world(self.world.clone())
    }

    pub fn on_leave_frame(&self) -> LeaveFrame {
        LeaveFrame::from_world(self.world.clone())
    }

    pub fn on_update(&self) -> Update {
        Update::from_world(self.world.clone())
    }

    pub fn on_render(&self) -> Render {
        Render::from_world(self.world.clone())
    }
}

