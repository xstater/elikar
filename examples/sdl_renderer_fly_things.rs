use elikar::{Elikar, ElikarStates};
use elikar::sdl_renderer::{Renderer, Color};
use std::sync::Arc;
use xblend::{RGBA,rgba};
use xecs::{System, World};
use elikar::events::PollEvents;
use std::cell::{Ref, RefMut};
use xecs::resource::Resource;
use elikar::sdl_renderer::point::Point;
use rand::random;
use elikar::sdl_renderer::sprite::Sprite;
use elikar::sdl_renderer::rect::Rect;
use elikar::mouse::event::button::Button::*;

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        if let Some(_) = events.quit {
            states.quit()
        }
    }
}

struct ShowFPS;
impl<'a> System<'a> for ShowFPS {
    type Resource = &'a ElikarStates;
    type Dependencies = ();

    fn update(&'a mut self, states : Ref<'a,ElikarStates>) {
        println!("fps:{}",states.actual_fps());
    }
}

#[derive(Debug,Copy, Clone)]
struct Velocity{
    vx : f32,
    vy : f32
}

struct CreatePointAtMousePosition;
impl<'a> System<'a> for CreatePointAtMousePosition {
    type Resource = (&'a mut World,&'a PollEvents);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (mut world,event) : (RefMut<'a,World>,Ref<'a,PollEvents>)) {
        for event in &event.mouse_button_down {
            let x : f32 = random();
            let y : f32 = random();
            if event.button == Right {
                world.create_entity()
                    .attach(Rect::new(
                        event.position.0 - 9,
                        event.position.1 - 9, 19,19))
                    .attach(rgba!(230,230,255,0))
                    .attach(Velocity {
                        vx: x * 10.0_f32 - 5.0_f32,
                        vy: y * 10.0_f32 - 5.0_f32
                    });
            } else {
                world.create_entity()
                    .attach(Point::new(event.position.0,event.position.1))
                    .attach(rgba!(255,255,255,0))
                    .attach(Velocity {
                        vx: x * 10.0_f32 - 5.0_f32,
                        vy: y * 10.0_f32 - 5.0_f32
                    });
            }
        }
    }
}

struct UpdatePosition;
impl<'a> System<'a> for UpdatePosition {
    type Resource = &'a mut World;
    type Dependencies = ();

    fn update(&'a mut self, world : RefMut<'a,World>) {
        for (point,v) in world.query::<(&mut Point,&Velocity)>() {
            point.x += v.vx as i32;
            point.y += v.vy as i32;
        }
        for (rect,v) in world.query::<(&mut Rect,&Velocity)>() {
            rect.x += v.vx as i32;
            rect.y += v.vy as i32;
        }
    }
}



fn main() {
    let mut game = Elikar::new().unwrap();
    let window = game.create_window().build().unwrap();
    let window = Arc::new(window);
    let renderer = Renderer::builder(window.clone())
        .accelerated()
        .vsync()
        .build()
        .unwrap();

    game.current_stage_mut().world_mut()
        .register::<Point>()
        .register::<Color>()
        .register::<Velocity>()
        .register::<Sprite>()
        .register::<Rect>();

    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(PollEvents::new())
        .add_system(renderer)
        .add_system(CreatePointAtMousePosition)
        .add_system(ShowFPS)
        .add_system(UpdatePosition)
    ;

    game.run();
}