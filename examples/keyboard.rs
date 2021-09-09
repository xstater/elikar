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

struct HandleKeyboardEvent;
impl<'a> System<'a> for HandleKeyboardEvent {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self,(events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        for key in &events.key_down {
            println!("{:?}",key);
            if key.code == Code::Escape {
                states.quit()
            }
        }
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
        .add_system(HandleKeyboardEvent);

    game.run();
}
