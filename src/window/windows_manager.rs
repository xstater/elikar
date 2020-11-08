use crate::window::Window;
use crate::Elikar;

pub struct WindowsManager{
    next_id : usize,
    windows : Vec<(usize,Window)>
}

impl WindowsManager {
    ///you must use this function after Elikar has been built
    pub fn new(_: &Elikar) -> WindowsManager {
        WindowsManager {
            next_id: 0,
            windows: Vec::new()
        }
    }

    pub fn add_windows(&mut self, window: Window) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.windows.push((id, window));
        id
    }

    pub fn take_window(&mut self, id: usize) -> Option<Window> {
        for (index, (id_, _)) in self.windows.iter().enumerate() {
            if *id_ == id {
                return Some(self.windows.remove(index).1);
            }
        }
        None
    }

    pub fn count(&self) -> usize {
        self.windows.len()
    }

    pub fn window(&self, index: usize) -> Option<&Window> {
        for (id,w) in &self.windows {
            if *id == index {
                return Some(w)
            }
        }
        None
    }
    pub fn window_mut(&mut self, index: usize) -> Option<&mut Window> {
        for (id,w) in &mut self.windows {
            if *id == index {
                return Some(w)
            }
        }
        None
    }

}

