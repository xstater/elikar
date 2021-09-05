
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Rect {
    pub x : i32,
    pub y : i32,
    pub w : u32,
    pub h : u32
}

impl From<SDL_Rect> for Rect {
    fn from(sdl_rect : SDL_Rect) -> Self {
        Rect {
            x : sdl_rect.x,
            y : sdl_rect.y,
            w : sdl_rect.w as _,
            h : sdl_rect.h as _
        }
    }
}

impl Rect {
    pub fn new(x : i32, y : i32, w : u32, h : u32) -> Self {
        Rect {
            x,
            y,
            w,
            h
        }
    }

    pub fn move_to(&mut self, x : i32, y : i32) {
        self.x = x;
        self.y = y;
    }

}