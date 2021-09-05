use crate::sdl_renderer::{Renderer, Color};
use xecs::{System, World};
use xecs::resource::Resource;
use sdl2_sys::*;
use sdl2_sys::SDL_RendererFlip::*;
use xecs::system::End;
use std::cell::Ref;
use crate::sdl_renderer::point::Point;
use crate::sdl_renderer::sprite::Sprite;
use std::ptr::null;
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
                SDL_RenderDrawRect(self.sdl_renderer,
                &SDL_Rect{
                    x : rect.x,
                    y : rect.y,
                    w : rect.w as _,
                    h : rect.h as _
                });
            }
        }

        // draw sprites
        for (sprite) in world.query::<&Sprite>() {
            let rect = SDL_Rect{
                x : sprite.position().0 as _,
                y : sprite.position().1 as _,
                w : sprite.size().0 as _,
                h : sprite.size().1 as _
            };
            unsafe {
                let flip = match (sprite.flip().0, sprite.flip().1) {
                    (false,false) => SDL_FLIP_NONE,
                    (true,false) => SDL_FLIP_HORIZONTAL,
                    (false,true) => SDL_FLIP_VERTICAL,
                    (true,true) => transmute::<u32, SDL_RendererFlip>
                        (SDL_FLIP_HORIZONTAL as u32 | SDL_FLIP_VERTICAL as u32)
                };
                SDL_RenderCopyEx(
                    self.sdl_renderer,
                    sprite.texture(),
                    null(),
                    &rect,
                    sprite.angle() as c_double,
                    null(),
                    flip
                );
            }
        }

        // present
        unsafe {
            SDL_RenderPresent(self.sdl_renderer);
        }
    }
}