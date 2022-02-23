use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use sdl2_sys::{SDL_MouseWheelDirection, SDL_MouseWheelEvent};
use xecs::{entity::EntityId, system::System, world::World};

#[derive(Debug, Clone, Copy)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: EntityId,
    pub which: u32,
    pub scrolled: (i32, i32),
}

impl EventInfo {
    pub(in crate) fn from_sdl_event(window_id : EntityId,event: SDL_MouseWheelEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            which: event.which,
            scrolled: if event.direction
                == SDL_MouseWheelDirection::SDL_MOUSEWHEEL_NORMAL as u32 {
                (event.x, event.y)
            } else {
                (-event.x, -event.y)
            },
        }
    }
}




pub(in crate) struct WheelInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct Wheel{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl Wheel {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        Wheel {
            world,
            rx : Option::None
        }
    }
}

impl Stream for Wheel{
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(WheelInner { tx, waker })
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

impl System for Wheel{
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for Wheel{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}

