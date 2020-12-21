extern crate sdl2_sys;

use sdl2_sys::*;
use crate::window::{Builder};
use std::sync::{RwLock, Arc};

pub(in crate) struct ManagerBase {
    pub(in crate) windows : Vec<*mut SDL_Window>
}

#[derive(Clone)]
pub struct Manager{
    base : Arc<RwLock<ManagerBase>>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            base : Arc::new(RwLock::new(ManagerBase{
                windows: Vec::new()
            }))
        }
    }

    pub fn builder(&self) -> Builder {
        Builder::from_windows_manager_base(Arc::downgrade(&self.base))
    }

    pub fn count(&self) -> usize{
        let guard = self.base.read().unwrap();
        guard.windows.len()
    }

}

impl Drop for ManagerBase{
    fn drop(&mut self) {
        for window in &mut self.windows {
            unsafe{
                SDL_DestroyWindow(*window);
            }
        }
    }
}

