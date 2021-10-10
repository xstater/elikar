use elikar::events::PollEvents;
use elikar::keyboard::Code;
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

struct PauseSystem(bool);
impl<'a> System<'a> for PauseSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents, &'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (events, mut states): (Ref<'a, PollEvents>, RefMut<'a, ElikarStates>),
    ) -> Result<(), Self::Error> {
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
        Ok(())
    }
}

struct PrintEventsSystem;
impl<'a> System<'a> for PrintEventsSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents, &'a ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (_events, states): (Ref<'a, PollEvents>, Ref<'a, ElikarStates>),
    ) -> Result<(), Self::Error> {
        println!("fps:{}", states.fps());
        Ok(())
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();
    {
        let mut manager = game
            .current_stage_ref()
            .system_data_mut::<window::Manager>();
        manager.create_window().build().unwrap();
    }
    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(PrintEventsSystem)
        .add_system(PauseSystem(true));

    game.run();
}
