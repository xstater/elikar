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
    ) -> Result<(), Infallible> {
        if let Some(_) = events.quit {
            states.quit()
        }
        Ok(())
    }
}

struct PrintEventsSystem;
impl<'a> System<'a> for PrintEventsSystem {
    type InitResource = ();
    type Resource = &'a PollEvents;
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, events: Ref<'a, PollEvents>) -> Result<(), Infallible> {
        for motion in &events.mouse_motion {
            println!("position:{:?}", motion.position)
        }
        for button in &events.mouse_button_down {
            println!("button:{:?}", button)
        }
        for wheel in &events.mouse_wheel {
            println!("wheel:{:?}", wheel);
        }
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
        .add_system(PrintEventsSystem);

    game.run();
}
