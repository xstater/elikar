use elikar::{Elikar, ElikarStates};
use xecs::{System};
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;
use elikar::keyboard::Code;
use std::convert::Infallible;

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) -> Result<(),Infallible>{
        if let Some(_) = events.quit {
            states.quit()
        }
        Ok(())
    }
}

struct HandleKeyboardEvent;
impl<'a> System<'a> for HandleKeyboardEvent {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self,(events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) -> Result<(),Infallible>{
        for key in &events.key_down {
            println!("{:?}",key);
            if key.code == Code::Escape {
                states.quit()
            }
        }
        Ok(())
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
