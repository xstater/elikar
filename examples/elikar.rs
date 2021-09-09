use elikar::{Elikar, ElikarStates};
use xecs::{System};
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;
use elikar::keyboard::Code;

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

struct PauseSystem(bool);
impl<'a> System<'a> for PauseSystem {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        for key in &events.key_down {
            if key.code == Code::P {
                if self.0 {
                    states.deactivate_system::<PrintEventsSystem>();
                    self.0 = false;
                } else {
                    states.activate_system::<PrintEventsSystem>();
                    self.0 = true
                }
            }
        }
    }
}

struct PrintEventsSystem;
impl<'a> System<'a> for PrintEventsSystem {
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self,(_events,states) : (Ref<'a,PollEvents>,Ref<'a,ElikarStates>)) {
        println!("fps:{}",states.fps());
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    let mut manager = game.create_window_manager();
    manager.create_window().build().unwrap();
    game.current_stage_mut()
        .add_system(manager)
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(PrintEventsSystem)
        .add_system(PauseSystem(true));

    game.run();
}