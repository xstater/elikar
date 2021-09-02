use elikar::{Elikar, ElikarStates};
use xecs::{System};
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;

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
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self,(events,states) : (Ref<'a,PollEvents>,Ref<'a,ElikarStates>)) {
        if let Some(_motion) = &events.mouse_motion {
            // println!("position:{:?}",motion.position);
            println!("fps:{}",states.fps());
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    let _window = game.create_window().build().unwrap();
    game.current_stage_mut()
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(PrintEventsSystem);

    game.run();
}