use std::collections::LinkedList;
use crate::window::{Window, WindowBuilder};

pub struct WindowsManager{
    windows : LinkedList<Window>
}

impl WindowsManager{
    pub fn new() -> WindowsManager{
        WindowsManager{
            windows : LinkedList::new()
        }
    }

    pub fn window_builder(&mut self) -> WindowBuilder{
        WindowBuilder::new(&mut self.windows)
    }

    pub fn count(&self) -> usize{
        self.windows.len()
    }

    pub fn find_by_id(&self,id : u32) -> Option<&Window>{
        for w in &self.windows{
            if w.id() == id {
                return Some(w)
            }
        }
        Option::None
    }

    pub fn find_by_id_mut(&mut self,id : u32) -> Option<&mut Window>{
        for w in &mut self.windows{
            if w.id() == id {
                return Some(w)
            }
        }
        Option::None
    }
}
