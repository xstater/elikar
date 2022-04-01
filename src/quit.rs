pub struct Quit{
    need_quit : bool
}

impl Quit {
    pub(in crate) fn new() -> Quit {
        Quit {
            need_quit : false
        }
    }

    pub fn need_quit(&self) -> bool {
        self.need_quit
    }

    pub fn quit(&mut self) {
        self.need_quit = true
    }
}



