use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use xecs::{entity::EntityId, system::System, world::World};

pub(in crate) struct UpdateInner{
    pub(in crate) tx : Sender<()>,
    pub(in crate) waker : Waker
}

pub struct Update{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<()>)>
}

impl Update{
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        Update{
            world,
            rx: Option::None,
        }
    }
}

impl Stream for Update{
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(UpdateInner{
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

impl System for Update {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for Update{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
