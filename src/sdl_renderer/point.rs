use sdl2_sys::*;
use crate::sdl_renderer::Renderer;
use xecs::{System, World};
use xecs::resource::Resource;
use std::cell::Ref;
use xecs::system::End;

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

}