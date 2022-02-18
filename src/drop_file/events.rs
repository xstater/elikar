use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use sdl2_sys::{SDL_DropEvent, SDL_free};
use xecs::{entity::EntityId, query::WithId, system::System, world::World};
use std::{ffi::CStr, path::PathBuf, pin::Pin, sync::{Arc, RwLock}, task::{Context, Poll, Waker}};
use crate::window::Window;

#[derive(Debug, Clone)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: EntityId,
    pub path: PathBuf,
}

impl EventInfo {
    pub(in crate) fn from_sdl_event(world : &World,event: SDL_DropEvent) -> Self{
        let window_id = world.query::<&Window>().with_id().find(|(_,window)|{
            window.id() == event.windowID
        }).map(|(id,_)|id)
        .expect("Drop Event:A drop event was sent from a non-existing window.");
        let path = PathBuf::from(
            unsafe { CStr::from_ptr(event.file) }
                .to_str()
                .unwrap());
        unsafe {
            SDL_free(event.file as *mut _);
        };
        EventInfo {
            timestamp : event.timestamp,
            window_id,
            path
        }
    }
}





pub(in crate) struct DropEventInner {
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct DropEvent {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl DropEvent {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        DropEvent {
            world,
            rx : Option::None
        }
    }
}

impl Stream for DropEvent{
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(DropEventInner{ tx, waker })
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

impl System for DropEvent {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for DropEvent{
    fn drop(&mut self) {
        let world = self.world.write().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
