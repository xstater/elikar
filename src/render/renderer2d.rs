use elikar_gl as gl;
use xecs::{System, World};
use xecs::resource::Resource;
use xecs::system::End;
use std::cell::RefMut;
use crate::window::Window;
use xblend::{ RGB,rgb };

pub struct Renderer2d;

impl<'a> System<'a> for Renderer2d {
    type Resource = &'a mut World;
    type Dependencies = End;

    fn update(&'a mut self, world : RefMut<'a,World>) {
        for window in world.query::<&mut Window>() {
            window.gl_swap();
        }
    }
}

pub struct ClearScreen{
    color : RGB<f32>
}

impl<'a> System<'a> for ClearScreen{
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self, _ : <Self::Resource as Resource<'a>>::Type) {
        unsafe {
            gl::ClearColor(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    }
}

impl ClearScreen {
    pub fn black() -> ClearScreen {
        ClearScreen{
            color : rgb!(0.0,0.0,0.0)
        }
    }
}