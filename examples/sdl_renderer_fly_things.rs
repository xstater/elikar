use elikar::events::PollEvents;
use elikar::mouse::event::button::Button::*;
use elikar::sdl_renderer::point::Point;
use elikar::sdl_renderer::rect::Rect;
use elikar::sdl_renderer::sprite::Sprite;
use elikar::sdl_renderer::{Color, Renderer};
use elikar::{window, Elikar, ElikarStates};
use rand::random;
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use xblend::{rgba, RGBA};
use xecs::{System, World};

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents, &'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (events, mut states): (Ref<'a, PollEvents>, RefMut<'a, ElikarStates>),
    ) -> Result<(), Infallible> {
        if let Some(_) = events.quit {
            states.quit()
        }
        Ok(())
    }
}

struct ShowFPS;
impl<'a> System<'a> for ShowFPS {
    type InitResource = ();
    type Resource = &'a ElikarStates;
    type Dependencies = ();
    type Error = Infallible;

    fn update(&'a mut self, states: Ref<'a, ElikarStates>) -> Result<(), Infallible> {
        println!("fps:{}", states.actual_fps());
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    vx: f32,
    vy: f32,
}

struct CreateThingsAtMousePosition;
impl<'a> System<'a> for CreateThingsAtMousePosition {
    type InitResource = ();
    type Resource = (&'a mut World, &'a PollEvents);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (mut world, event): (RefMut<'a, World>, Ref<'a, PollEvents>),
    ) -> Result<(), Infallible> {
        for event in &event.mouse_button_down {
            let x: f32 = random();
            let y: f32 = random();
            if event.button == Right {
                world
                    .create_entity()
                    .attach(Rect::new(
                        event.position.0 - 9,
                        event.position.1 - 9,
                        19,
                        19,
                    ))
                    .attach(rgba!(230, 230, 255, 0))
                    .attach(Velocity {
                        vx: x * 10.0_f32 - 5.0_f32,
                        vy: y * 10.0_f32 - 5.0_f32,
                    });
            } else {
                world
                    .create_entity()
                    .attach(Point::new(event.position.0, event.position.1))
                    .attach(rgba!(255, 255, 255, 0))
                    .attach(Velocity {
                        vx: x * 10.0_f32 - 5.0_f32,
                        vy: y * 10.0_f32 - 5.0_f32,
                    });
            }
        }
        Ok(())
    }
}

struct UpdatePosition;
impl<'a> System<'a> for UpdatePosition {
    type InitResource = ();
    type Resource = &'a mut World;
    type Dependencies = ();
    type Error = Infallible;

    fn update(&'a mut self, world: RefMut<'a, World>) -> Result<(), Infallible> {
        for (point, v) in world.query::<(&mut Point, &Velocity)>() {
            point.x += v.vx as i32;
            point.y += v.vy as i32;
        }
        for (rect, v) in world.query::<(&mut Rect, &Velocity)>() {
            rect.x += v.vx as i32;
            rect.y += v.vy as i32;
        }
        Ok(())
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();
    let renderer = {
        Renderer::builder()
            .accelerated()
            .vsync()
            .build(
                game.current_stage_ref()
                    .system_data_mut::<window::Manager>()
                    .create_window()
                    .build()
                    .unwrap(),
            )
            .unwrap()
    };

    game.current_stage_mut()
        .world_mut()
        .register::<Point>()
        .register::<Color>()
        .register::<Velocity>()
        .register::<Sprite>()
        .register::<Rect>();

    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(renderer)
        .add_system(CreateThingsAtMousePosition)
        .add_system(ShowFPS)
        .add_system(UpdatePosition);

    game.run();
}
