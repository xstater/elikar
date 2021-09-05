use crate::sdl_renderer::{Renderer, Color};
use xecs::{System, World};
use xecs::resource::Resource;
use sdl2_sys::*;
use xecs::system::End;
use std::cell::Ref;
use crate::sdl_renderer::point::Point;
use crate::sdl_renderer::sprite::{Sprite, ExInfo};
use std::ptr::null;
use sdl2_sys::sys::SDL_RendererFlip;
use std::mem::transmute;
use std::os::raw::c_double;
use crate::sdl_renderer::rect::Rect;

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
                SDL_RenderDrawPoint(self.sdl_renderer, point.x as _, point.y as _);
            }
        }

        // draw all rects
        for (color,rect) in world.query::<(&Color,&Rect)>() {
            // set color
            unsafe {
                SDL_SetRenderDrawColor(
                    self.sdl_renderer,
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a());
                SDL_RenderDrawRect(self.sdl_renderer, rect.into())
            }
        }

        // draw sprites
        for (sprite,ex_info) in world.query::<(&Sprite,&ExInfo)>() {
            let rect = SDL_Rect{
                x : sprite.position().0 as _,
                y : sprite.position().1 as _,
                w : sprite.size().0 as _,
                h : sprite.size().1 as _
            };
            unsafe {
                let flip = match (ex_info.flip().0, ex_info.flip().1) {
                    (false,false) => SDL_FLIP_NONE,
                    (true,false) => SDL_FLIP_HORIZONTAL,
                    (false,true) => SDL_FLIP_VERTICAL,
                    (true,true) => transmute::<u32, sys::SDL_RendererFlip>(
                                transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_HORIZONTAL) |
                                   transmute::<sys::SDL_RendererFlip, u32>(SDL_FLIP_VERTICAL)
                    )
                };
                SDL_RenderCopyEx(
                    self.sdl_renderer,
                    sprite.texture(),
                    null(),
                    &rect,
                    ex_info.angle() as c_double,
                    null(),
                    &flip
                );
            }
        }

        // present
        unsafe {
            SDL_RenderPresent(self.sdl_renderer);
        }
    }
}