use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crate::{mouse::ButtonState, window::Window};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use sdl2_sys::SDL_MouseMotionEvent;
use xecs::{entity::EntityId, query::WithId, system::System, world::World};

#[derive(Debug, Clone, Copy)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: EntityId,
    pub which: u32,
    pub state: ButtonState,
    pub position: (i32, i32),
    pub relative: (i32, i32),
}

impl EventInfo {
    pub(in crate) fn from_sdl_event(world : &World,event: SDL_MouseMotionEvent) -> Self {
        let window_id = world.query::<&Window>().with_id().find(|(_,window)|{
            window.id() == event.windowID
        }).map(|(id,_)|id)
        .expect("Mouse Motion Event: A mouse motion event was sent from a non-existing window.");
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            which: event.which,
            state: ButtonState::new(event.state),
            position: (event.x, event.y),
            relative: (event.xrel,event.yrel),
        }
    }
}




pub(in crate) struct MotionInner{
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct Motion {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl Motion{
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        Motion{
            world,
            rx : Option::None
        }
    }
}

impl Stream for Motion {
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(MotionInner{ tx, waker })
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

impl System for Motion {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for Motion{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
