use elikar::{Elikar, ElikarStates};
use xecs::{System, World};
use xecs::resource::Resource;
use std::cell::{RefMut, Ref};
use elikar::window::Window;
use elikar::events::PollEvents;

struct CreateWindowSystem;
impl<'a> System<'a> for CreateWindowSystem {
    type Resource = &'a mut World;
    type Dependencies = ();

    fn update(&'a mut self, mut world : RefMut<'a,World>) {
        world.register::<Window>();

        world.create_entity()
            .attach(elikar::window::Builder::default()
                .title("elikar test")
                .build()
                .unwrap());


    }
}

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

struct PrintEventsSystem;
impl<'a> System<'a> for PrintEventsSystem {
    type Resource = &'a PollEvents;
    type Dependencies = PollEvents;

    fn update(&'a mut self,events : Ref<'a,PollEvents>) {
        if let Some(motion) = &events.mouse_motion {
            println!("position:{:?}",motion.position)
        }
        if let Some(button) = &events.mouse_button_down {
            println!("button:{:?}",button)
        }
        if let Some(wheel) = &events.mouse_wheel {
            println!("wheel:{:?}",wheel);
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    game.current_stage_mut()
        .add_once_system(CreateWindowSystem);
    game.current_stage_mut()
        .add_system(PollEvents::new());
    game.current_stage_mut()
        .add_system(QuitSystem);
    game.current_stage_mut()
        .add_system(PrintEventsSystem);

    game.run();
}
