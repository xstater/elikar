use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use sdl2_sys::SDL_TextEditingEvent;
use xecs::{EntityId,System, World};
use crate::common::from_sdl_string;

#[derive(Debug,Clone)]
pub struct EventInfo {
    pub timestamp : u32,
    pub window_id : EntityId,
    pub text : String,
    pub start : u32,
    pub lenght : u32
}


impl EventInfo {
    pub(in crate) fn from_sdl_event(window_id : EntityId,event: SDL_TextEditingEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            text: unsafe { from_sdl_string(&event.text as *const _) },
            start: event.start as _,
            lenght: event.length as _
        }
    }
}





pub(in crate) struct TextEditingInner{
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct TextEditing{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl TextEditing{
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        TextEditing{
            world,
            rx : Option::None
        }
    }
}

impl Stream for TextEditing{
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(TextEditingInner{ tx, waker })
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

impl System for TextEditing{
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for TextEditing{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
