use std::{pin::Pin, sync::{Arc, RwLock}, task::{Context, Poll, Waker}};

use crate::{keyboard::{Code, Mod}, window::Window};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use sdl2_sys::{SDL_KeyboardEvent, SDL_PRESSED};
use xecs::{entity::EntityId, query::WithId, system::System, world::World};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum State {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: EntityId,
    pub state: State,
    pub is_repeat: bool,
    pub code: Code,
    pub mod_state: Mod,
}

impl EventInfo {
    pub(in crate) fn from_sdl_event(world : &World,event: SDL_KeyboardEvent) -> Self {
        let window_id = world.query::<&Window>().with_id().find(|(_,window)|{
            window.id() == event.windowID
        }).map(|(id,_)|id)
        .expect("Keyboard Event:A keyboard event was sent from a non-existing window.");
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            state: if event.state == SDL_PRESSED as u8 {
                State::Pressed
            } else {
                State::Released
            },
            is_repeat: event.repeat != 0,
            code: event.keysym.scancode.into(),
            mod_state: Mod::new(event.keysym.mod_ as u32),
        }
    }
}





pub(in crate) struct KeyDownInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct KeyDown {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl KeyDown {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        KeyDown{
            world,
            rx : Option::None
        }
    }
}

impl Stream for KeyDown{
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(KeyDownInner{ tx, waker })
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

impl System for KeyDown {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for KeyDown{
    fn drop(&mut self) {
        let world = self.world.read().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}






pub(in crate) struct KeyUpInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct KeyUp{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl KeyUp {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        KeyUp {
            world,
            rx : Option::None
        }
    }
}

impl Stream for KeyUp {
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(KeyUpInner{ tx, waker })
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

impl System for KeyUp{
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for KeyUp{
    fn drop(&mut self) {
        let world = self.world.read().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
