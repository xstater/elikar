pub struct Sprite{
    size : (u32,u32)
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            size: (0, 0)
        }
    }

    pub fn size(&self) -> (u32,u32) {
        self.size
    }

    pub fn set_size(&mut self,w : u32,h : u32) {
        self.size = (w,h)
    }
}