use elikar::{Elikar, ElikarStates};
use xecs::{System, World};
use xecs::resource::Resource;
use std::cell::{RefMut, Ref};
use elikar::window::Window;
use elikar::events::PollEvents;

struct CreateWindowSystem(Option<Window>);
impl<'a> System<'a> for CreateWindowSystem {
    type Resource = &'a mut World;
    type Dependencies = ();

    fn update(&'a mut self, mut world : RefMut<'a,World>) {
        world.register::<Window>();

        world.create_entity()
            .attach(self.0.take().unwrap());


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
        if let Some(files) = &events.drop_files {
            for path in &files.paths {
                println!("{:?}",path);
            }
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();

    let window = game.create_window().build().unwrap();

    game.current_stage_mut()
        .add_once_system(CreateWindowSystem(Some(window)))
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(PrintEventsSystem);

    game.run();
}
