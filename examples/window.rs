use elikar::{Elikar, ElikarStates};
use xecs::System;
use elikar::events::PollEvents;
use std::cell::{Ref, RefMut};

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

struct PrintWindowEvent;
impl<'a> System<'a> for PrintWindowEvent {
    type Resource = &'a PollEvents;
    type Dependencies = PollEvents;

    fn update(&'a mut self,events : Ref<'a,PollEvents>) {
        for event in &events.window {
            println!("Window Event:{:?}", event)
        }
    }
}
fn main() {
    let mut game = Elikar::new().unwrap();

    let mut window = game.create_window()
        .build()
        .unwrap();
    window.set_resizable();

    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(PollEvents::new())
        .add_system(PrintWindowEvent);

    game.run();
}

