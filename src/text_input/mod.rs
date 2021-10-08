pub mod event;

use std::convert::Infallible;
use sdl2_sys::*;
use xecs::System;

pub struct TextInput {
}

impl TextInput {
    pub(in crate) fn new() -> TextInput {
        TextInput {}
    }

    pub fn start(&mut self) {
        unsafe {
            SDL_StartTextInput()
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            SDL_StopTextInput()
        }
    }

    pub fn set_area(&mut self,x : i32,y : i32,width : u32,height : u32) {
        let mut rect = SDL_Rect {
            x,y,
            w : width as _,
            h : height as _
        };
        unsafe {
            SDL_SetTextInputRect(&mut rect)
        }
    }

    pub fn active(&self) -> bool {
        unsafe {
            SDL_IsTextInputActive() == SDL_bool::SDL_TRUE
        }
    }
}

impl<'a> System<'a> for TextInput {
    type InitResource = ();
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;
}