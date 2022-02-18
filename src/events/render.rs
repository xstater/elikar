use std::{pin::Pin, sync::{Arc, RwLock}, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use xecs::{entity::EntityId, system::System, world::World};

pub(in crate) struct RenderInner{
    pub(in crate) tx : Sender<()>,
    pub(in crate) waker : Waker
}

pub struct Render{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<()>)>
}

impl Render{
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        Render{
            world,
            rx: Option::None,
        }
    }
}

impl Stream for Render{
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read().unwrap();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(RenderInner{
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

impl System for Render {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for Render{
    fn drop(&mut self) {
        let world = self.world.read().unwrap();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
