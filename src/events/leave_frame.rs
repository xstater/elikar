use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use xecs::{component::Component, entity::EntityId, system::System, world::World};

pub(in crate) struct LeaveFrameInner {
    pub(in crate) tx : Sender<u32>,
    pub(in crate) waker : Waker
}

impl Component for LeaveFrameInner {}

pub struct LeaveFrame {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<u32>)>
}

impl LeaveFrame {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        LeaveFrame {
            world,
            rx: Option::None,
        }
    }
}

impl Stream for LeaveFrame {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(LeaveFrameInner {
                        tx,
                        waker
                    }).into_id()
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

impl System for LeaveFrame {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for LeaveFrame{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
