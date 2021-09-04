use crate::sdl_renderer::{Renderer, Color};
use xecs::{System, World};
use xecs::resource::Resource;
use sdl2_sys::*;
use xecs::system::End;
use std::cell::Ref;
use crate::sdl_renderer::point::Point;

impl<'a> System<'a> for Renderer {
    type Resource = &'a World;
    type Dependencies = End;

    fn update(&'a mut self, world : Ref<'a,World>) {
        // clear
        unsafe {
            SDL_SetRenderDrawColor(
                self.sdl_renderer,
                self.clear_color.r(),
                self.clear_color.g(),
                self.clear_color.b(),
                self.clear_color.a());
            SDL_RenderClear(self.sdl_renderer);
        }

        // draw all points
        for (color,point) in world.query::<(&Color,&Point)>() {
            // set color
            unsafe {
                SDL_SetRenderDrawColor(
                    self.sdl_renderer,
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a());
                SDL_RenderDrawPoint(self.sdl_renderer,point.x as _,point.y as _);
            }
        }

        // present
        unsafe {
            SDL_RenderPresent(self.sdl_renderer);
        }
    }
}