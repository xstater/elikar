use std::{pin::Pin, sync::Arc, task::{Context, Poll, Waker}};
use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::Stream;
use parking_lot::RwLock;
use sdl2_sys::*;
use xecs::{entity::EntityId, system::System, world::World};

#[derive(Debug, Copy, Clone)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id : EntityId,
    pub event_type: WindowEventType,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum WindowEventType {
    None,
    /// Window has been shown
    Shown,
    /// Window has been hidden
    Hidden,
    /// Window has been exposed and should be redrawn
    Exposed,
    /// Window has been moved to (x,y)
    Moved(u32, u32),
    /// Window has been resized to (w,h) ,this event is always preceded by WindowEvent::SizeChanged
    Resized(u32, u32),
    /// Window size has changed,either as a result of an API call or through the system or user
    /// changing the window size; this event is followed by WindowEvent::Resized if the size was
    /// changed by an external event, i.e. the user or the window manager.
    SizeChanged(u32, u32),
    /// Window has been minimized
    Minimized,
    /// Window has been maximized
    Maximized,
    /// Window has been restored to normal size and position
    Restored,
    /// Window has gained mouse focus
    Enter,
    /// Window has lost mouse focus
    Leave,
    /// Window has gained keyboard focus
    FocusGained,
    /// Window has lost keyboard focus
    FocusLost,
    /// The window manager requests that the window be closed
    Close,
    /// Window is being offered a focus
    TakeFocus,
}

impl EventInfo {
    pub(in crate) fn from_sdl_event(window_id : EntityId,event : SDL_WindowEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id,
            event_type : match event.event as u32 {
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_SHOWN as u32 => WindowEventType::Shown,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_HIDDEN as u32 => WindowEventType::Hidden,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_EXPOSED as u32 => WindowEventType::Exposed,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MOVED as u32 => {
                    WindowEventType::Moved(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_RESIZED as u32 => {
                    WindowEventType::Resized(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_SIZE_CHANGED as u32 => {
                    WindowEventType::SizeChanged(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MINIMIZED as u32 => {
                    WindowEventType::Minimized
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MAXIMIZED as u32 => {
                    WindowEventType::Maximized
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_RESTORED as u32 => {
                    WindowEventType::Restored
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_ENTER as u32 => WindowEventType::Enter,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_LEAVE as u32 => WindowEventType::Leave,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_GAINED as u32 => {
                    WindowEventType::FocusGained
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_LOST as u32 => {
                    WindowEventType::FocusLost
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_CLOSE as u32 => WindowEventType::Close,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_TAKE_FOCUS as u32 => {
                    WindowEventType::TakeFocus
                }
                _ => WindowEventType::None,
            },
        }
    }
}

pub(in crate) struct WindowEventInner{
    pub(in crate) tx : Sender<EventInfo>,
    pub(in crate) waker : Waker
}

pub struct WindowEvent{
    world : Arc<RwLock<World>>,
    rx : Option<(EntityId,Receiver<EventInfo>)>
}

impl WindowEvent {
    pub(in crate) fn from_world(world : Arc<RwLock<World>>) -> Self {
        WindowEvent{
            world,
            rx : Option::None
        }
    }
}

impl Stream for WindowEvent {
    type Item = EventInfo;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_none() {
            let (tx,rx) = unbounded();
            let id = {
                let world = self.world.read();
                let waker = cx.waker().clone();
                world.create_entity()
                    .attach(WindowEventInner{ tx, waker })
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

impl System for WindowEvent {
    fn world(&self) -> Arc<RwLock<World>> {
        self.world.clone()
    }
}

impl Drop for WindowEvent{
    fn drop(&mut self) {
        let world = self.world.read();
        if let Some((id,_)) = self.rx {
            world.remove_entity(id)
        }
    }
}
