use elikar::{Elikar, ElikarStates};
use elikar::sdl_renderer::{Renderer, Color};
use std::sync::Arc;
use elikar::sdl_renderer::sprite::Sprite;
use xecs::{System, World};
use elikar::events::PollEvents;
use std::cell::{Ref, RefMut};
use elikar::sdl_renderer::point::Point;
use xecs::resource::Resource;

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
    type Resource = (&'a PollEvents,&'a mut World);
    type Dependencies = ();

    fn update(&'a mut self, (events,mut world) : (Ref<'a,PollEvents>,RefMut<'a,World>)) {
        if let Some(motion) = events.mouse_motion {
            for sprite in world.query::<&mut Sprite>() {
                sprite.move_to(motion.position);
            }
        }
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();
    let window =  game.create_window().build().unwrap();
    let window = Arc::new(window);
    let renderer = Renderer::builder(window)
        .accelerated()
        .build()
        .unwrap();

    let sprite = Sprite::from_bmp(&renderer,"./logo.bmp").unwrap();

    game.current_stage_mut()
        .world_mut()
        .register::<Sprite>()
        .register::<Point>()
        .register::<Color>();

    game.current_stage_mut()
        .world_mut()
        .create_entity()
        .attach(sprite);

    game.current_stage_mut()
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(ShowFPS)
        .add_system(renderer)
        .add_system(FollowMouse);

    game.run();
}