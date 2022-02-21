use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use xecs::{component::Component, entity::EntityId, system::System, world::World};

pub(in super) struct QuitEvent {
    pub(in super) tx : Sender<()>,
    pub(in super) waker : Waker
}

impl Component for QuitEvent {}

pub struct Quit {
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<()>)>,
}

impl Quit {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        Quit {
            world,
            rx : Option::None
        }
    }
}

impl Stream for Quit {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(QuitEvent {
                        tx,
                        waker
                    }).into_id()
            };
            self.rx.replace((id,rx));
        }
        let (_,rx) = self.rx.as_ref().unwrap();
        if let Ok(_) = rx.try_recv() {
            Poll::Ready(Some(()))
        } else {
            Poll::Pending
        }
    }
}

impl System for Quit {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for Quit {
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
