use std::{pin::Pin, sync::{Arc, RwLock}, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use sdl2_sys::SDL_MouseButtonEvent;
use xecs::{entity::EntityId, query::WithId, system::System, world::World};
use crate::window::Window;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Clicks {
    Single,
    Double,
}

#[derive(Debug, Copy, Clone)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: EntityId,
    pub button: Button,
    pub click: Clicks,
    pub position: (i32, i32),
}

const BUTTON_ID_LEFT: u8 = 1;
const BUTTON_ID_MIDDLE: u8 = 2;
const BUTTON_ID_RIGHT: u8 = 3;
const BUTTON_ID_X1: u8 = 4;
const BUTTON_ID_X2: u8 = 5;

impl EventInfo {
    pub(in crate) fn from_sdl_event(world : &World,event: SDL_MouseButtonEvent) -> Self {
        let window_id = world.query::<&Window>().with_id().find(|(_,window)|{
            window.id() == event.windowID
        }).map(|(id,_)|id)
        .expect("Mouse Button Event:A mouse button event was sent from a non-existing window.");
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            button: match event.button {
                BUTTON_ID_LEFT => Button::Left,
                BUTTON_ID_MIDDLE => Button::Middle,
                BUTTON_ID_RIGHT => Button::Right,
                BUTTON_ID_X1 => Button::X1,
                BUTTON_ID_X2 => Button::X2,
                _ => Button::Left,
            },
            click: if event.clicks == 1 {
                Clicks::Single
            } else {
                Clicks::Double
            },
            position: (event.x, event.y),
        }
    }
}




pub(in crate) struct ButtonDownInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct ButtonDown {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl ButtonDown {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        ButtonDown {
            world,
            rx : Option::None
        }
    }
}

impl Stream for ButtonDown {
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(ButtonDownInner{ tx, waker })
                    .into_id()
            };
            self.rx.replace((id,rx));
        }
        let (_,rx) = self.rx.as_ref().unwrap();
        if let Ok(info) = rx.try_recv() {
            Poll::Ready(Some(info))
        } else {
            Poll::Pending
        }
    }
}

impl System for ButtonDown {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for ButtonDown {
    fn drop(&mut self) {
        let world = self.world.read().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}






pub(in crate) struct ButtonUpInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct ButtonUp {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl ButtonUp {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        ButtonUp {
            world,
            rx : Option::None
        }
    }
}

impl Stream for ButtonUp{
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(ButtonUpInner{ tx, waker })
                    .into_id()
            };
            self.rx.replace((id,rx));
        }
        let (_,rx) = self.rx.as_ref().unwrap();
        if let Ok(info) = rx.try_recv() {
            Poll::Ready(Some(info))
        } else {
            Poll::Pending
        }
    }
}

impl System for ButtonUp{
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for ButtonUp{
    fn drop(&mut self) {
        let world = self.world.read().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
