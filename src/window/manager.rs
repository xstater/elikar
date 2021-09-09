use crate::window::{Window, Builder};
use xecs::System;

pub struct Manager {
    windows : Vec<Window>
}

impl Manager{
    pub fn new() -> Manager {
        Manager{
            windows: vec![]
        }
    }

    pub fn create_window(&mut self) -> Builder<'_> {
        Builder::from_manager(self)
    }

    pub(in crate) fn add_window(&mut self,window : Window){
        self.windows.push(window);
    }

    pub fn window_ref(&self, window_id : u32) -> Option<&Window> {
        self.windows.iter()
            .find(|window|{
                window.id().unwrap() == window_id
            })
    }

    pub fn window_mut(&mut self, window_id : u32) -> Option<&mut Window> {
        self.windows.iter_mut()
            .find(|window|{
                window.id().unwrap() == window_id
            })
    }

    pub fn remove_window(&mut self,window_id : u32) -> Option<Window> {
        let index = self.windows.iter()
            .enumerate()
            .find(|(_,window)|{
                window.id().unwrap() == window_id
            })
            .map(|(index,_)|index)?;
        Some(self.windows.remove(index))
    }

    pub fn iter(&self) -> impl Iterator<Item=&Window> {
        self.windows.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Window> {
        self.windows.iter_mut()
    }
}

impl<'a> System<'a> for Manager {
    type Resource = ();
    type Dependencies = ();
}