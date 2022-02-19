use std::sync::Arc;
use crate::{drop_file, ime, keyboard, mouse, window};
use parking_lot::RwLock;
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
            let mut world = world.write();
            world.register::<QuitEvent>()
                .register::<mouse::events::button::ButtonDownInner>()
                .register::<mouse::events::button::ButtonUpInner>()
                .register::<mouse::events::motion::MotionInner>()
                .register::<mouse::events::wheel::WheelInner>()
                .register::<window::events::WindowEventInner>()
                .register::<keyboard::events::KeyDownInner>()
                .register::<keyboard::events::KeyUpInner>()
                .register::<drop_file::events::DropEventInner>()
                .register::<ime::events::text_input::TextInputInner>()
                .register::<ime::events::text_editing::TextEditingInner>()
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
            let world = self.world.read();
            match unsafe { sdl_event.type_ } {
                x if x == SDL_EventType::SDL_QUIT as u32 => {
                    for quit_event in world.query::<&QuitEvent>() {
                        // This will never fails
                        quit_event.tx.send(()).unwrap();
                        quit_event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 => {
                    let event_info = mouse::events::button::EventInfo::from_sdl_event(&world,unsafe { sdl_event.button });
                    for event in world.query::<&mouse::events::button::ButtonDownInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEBUTTONUP as u32 => {
                    let event_info = mouse::events::button::EventInfo::from_sdl_event(&world,unsafe { sdl_event.button });
                    for event in world.query::<&mouse::events::button::ButtonUpInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEMOTION as u32 => {
                    let event_info = mouse::events::motion::EventInfo::from_sdl_event(&world,unsafe { sdl_event.motion });
                    for event in world.query::<&mouse::events::motion::MotionInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_MOUSEWHEEL as u32 => {
                    let event_info = mouse::events::wheel::EventInfo::from_sdl_event(&world, unsafe { sdl_event.wheel });
                    for event in world.query::<&mouse::events::wheel::WheelInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                    let event_info = keyboard::events::EventInfo::from_sdl_event(&world, unsafe { sdl_event.key });
                    for event in world.query::<&keyboard::events::KeyDownInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_KEYUP as u32 => {
                    let event_info = keyboard::events::EventInfo::from_sdl_event(&world, unsafe { sdl_event.key });
                    for event in world.query::<&keyboard::events::KeyUpInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_WINDOWEVENT as u32 => {
                    let event_info = window::events::EventInfo::from_sdl_event(&world, unsafe { sdl_event.window });
                    for event in world.query::<&window::events::WindowEventInner>() {
                        event.tx.send(event_info).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_DROPBEGIN as u32 => {
                },
                x if x == SDL_EventType::SDL_DROPFILE as u32 => {
                    let event_info = drop_file::events::EventInfo::from_sdl_event(&world,unsafe { sdl_event.drop });
                    for event in world.query::<&drop_file::events::DropEventInner>() {
                        event.tx.send(event_info.clone()).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_DROPCOMPLETE as u32 => {
                },
                x if x == SDL_EventType::SDL_TEXTEDITING as u32 => {
                    let event_info = ime::events::text_editing::EventInfo::from_sdl_event(&world,unsafe{ sdl_event.edit });
                    for event in world.query::<&ime::events::text_editing::TextEditingInner>() {
                        event.tx.send(event_info.clone()).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                x if x == SDL_EventType::SDL_TEXTINPUT as u32 => {
                    let event_info = ime::events::text_input::EventInfo::from_sdl_event(&world, unsafe{ sdl_event.text });
                    dbg!(&event_info);
                    for event in world.query::<&ime::events::text_input::TextInputInner>() {
                        event.tx.send(event_info.clone()).unwrap();
                        event.waker.wake_by_ref();
                    }
                },
                _ => {}
            }
        }
    }

    pub fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }

    pub fn on_quit(&self) -> Quit {
        Quit::from_world(self.world.clone())
    }

    pub fn on_mouse_down(&self) -> mouse::events::button::ButtonDown {
        mouse::events::button::ButtonDown::from_world(self.world.clone())
    }

    pub fn on_mouse_up(&self) -> mouse::events::button::ButtonUp{
        mouse::events::button::ButtonUp::from_world(self.world.clone())
    }

    pub fn on_mouse_motion(&self) -> mouse::events::motion::Motion {
        mouse::events::motion::Motion::from_world(self.world.clone())
    }

    pub fn on_mouse_wheel(&self) -> mouse::events::wheel::Wheel {
        mouse::events::wheel::Wheel::from_world(self.world.clone())
    }

    pub fn on_key_down(&self) -> keyboard::events::KeyDown {
        keyboard::events::KeyDown::from_world(self.world.clone())
    }

    pub fn on_key_up(&self) -> keyboard::events::KeyUp {
        keyboard::events::KeyUp::from_world(self.world.clone())
    }

    pub fn on_window_events(&self) -> window::events::WindowEvent {
        window::events::WindowEvent::from_world(self.world.clone())
    }

    pub fn on_drop_file(&self) -> drop_file::events::DropEvent {
        drop_file::events::DropEvent::from_world(self.world.clone())
    }

    pub fn on_text_editing(&self) -> ime::events::text_editing::TextEditing {
        ime::events::text_editing::TextEditing::from_world(self.world.clone())
    }

    pub fn on_text_input(&self) -> ime::events::text_input::TextInput {
        ime::events::text_input::TextInput::from_world(self.world.clone())
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

