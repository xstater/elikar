use elikar::events::PollEvents;
use elikar::{window, Elikar, ElikarStates};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use xecs::System;

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents, &'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (events, mut states): (Ref<'a, PollEvents>, RefMut<'a, ElikarStates>),
    ) -> Result<(), Self::Error> {
        if let Some(_) = events.quit {
            states.quit()
        }
        Ok(())
    }
}

struct PrintWindowEvent;
impl<'a> System<'a> for PrintWindowEvent {
    type InitResource = ();
    type Resource = &'a PollEvents;
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, events: Ref<'a, PollEvents>) -> Result<(), Self::Error> {
        for event in &events.window {
            println!("Window Event:{:?}", event)
        }
        Ok(())
    }
}
fn main() {
    let mut game = Elikar::new().unwrap();

    {
        let mut manager = game
            .current_stage_mut()
            .system_data_mut::<window::Manager>();
        let window = manager
            .create_window()
            .resizable()
            .always_on_top()
            .skip_taskbar()
            .title("window event test")
            .build()
            .unwrap();
        dbg!(window.id());
    }

    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(PrintWindowEvent);

    game.run();
}
