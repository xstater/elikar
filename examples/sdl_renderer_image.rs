use elikar::{Elikar, ElikarStates};
use elikar::sdl_renderer::{Renderer, Color};
use elikar::sdl_renderer::sprite::Sprite;
use xecs::{System, World};
use elikar::events::PollEvents;
use std::cell::{Ref, RefMut};
use elikar::sdl_renderer::point::Point;
use elikar::sdl_renderer::rect::Rect;

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

struct FollowMouse;
impl<'a> System<'a> for FollowMouse {
    type Resource = (&'a PollEvents,&'a World);
    type Dependencies = ();

    fn update(&'a mut self, (events,world) : (Ref<'a,PollEvents>,Ref<'a,World>)) {
        for motion in &events.mouse_motion {
            for sprite in world.query::<&mut Sprite>() {
                sprite.move_to(motion.position);
            }
        }
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();
    let mut manager = game.create_window_manager();
    let renderer = {
        Renderer::builder()
            .accelerated()
            .build(manager.create_window()
                .build()
                .unwrap())
            .unwrap()
    };

    let mut sprite = Sprite::from_bmp(&renderer,"./logo.bmp").unwrap();
    sprite.set_angle(180.0);
    sprite.set_flip(true,true);

    game.current_stage_mut()
        .world_mut()
        .register::<Sprite>()
        .register::<Point>()
        .register::<Color>()
        .register::<Rect>();

    game.current_stage_mut()
        .world_mut()
        .create_entity()
        .attach(sprite);

    game.current_stage_mut()
        .add_system(manager)
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(ShowFPS)
        .add_system(renderer)
        .add_system(FollowMouse);

    game.run();
}