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
        if let Some(key) = events.key_down {
            println!("{:?}",key);
            if key.code == Code::Escape {
                states.quit()
            }
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    let _window = game.create_window().build().unwrap();
    game.current_stage_mut()
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(HandleKeyboardEvent);

    game.run();
}
