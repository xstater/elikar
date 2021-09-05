use sdl2_sys::*;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Point {
    pub x : i32,
    pub y : i32
}

impl From<SDL_Point> for Point {
    fn from(sdl_point: SDL_Point) -> Self {
        Point{
            x : sdl_point.x,
            y : sdl_point.y
        }
    }
}

impl Point{
    pub fn new(x : i32,y : i32) -> Self {
        Point {
            x,
            y
        }
    }

    pub fn move_to(&mut self, x : i32, y : i32) {
        self.x = x;
        self.y = y;
    }

}