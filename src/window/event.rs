use crate::window::WindowId;
use sdl2_sys::*;

#[derive(Debug, Copy, Clone)]
pub struct EventInfo {
    pub timestamp: u32,
    pub window_id: WindowId,
    pub event: WindowEvent,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
    ///
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

impl From<SDL_WindowEvent> for EventInfo {
    fn from(event: SDL_WindowEvent) -> Self {
        EventInfo {
            timestamp: event.timestamp,
            window_id: WindowId::from_u32(event.windowID),
            event: match event.event as u32 {
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_SHOWN as u32 => WindowEvent::Shown,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_HIDDEN as u32 => WindowEvent::Hidden,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_EXPOSED as u32 => WindowEvent::Exposed,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MOVED as u32 => {
                    WindowEvent::Moved(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_RESIZED as u32 => {
                    WindowEvent::Resized(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_SIZE_CHANGED as u32 => {
                    WindowEvent::SizeChanged(event.data1 as _, event.data2 as _)
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MINIMIZED as u32 => {
                    WindowEvent::Minimized
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_MAXIMIZED as u32 => {
                    WindowEvent::Maximized
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_RESTORED as u32 => {
                    WindowEvent::Restored
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_ENTER as u32 => WindowEvent::Enter,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_LEAVE as u32 => WindowEvent::Leave,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_GAINED as u32 => {
                    WindowEvent::FocusGained
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_FOCUS_LOST as u32 => {
                    WindowEvent::FocusLost
                }
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_CLOSE as u32 => WindowEvent::Close,
                x if x == SDL_WindowEventID::SDL_WINDOWEVENT_TAKE_FOCUS as u32 => {
                    WindowEvent::TakeFocus
                }
                _ => WindowEvent::None,
            },
        }
    }
}
