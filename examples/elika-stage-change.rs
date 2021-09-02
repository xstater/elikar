use elikar::{Elikar, ElikarStates};
use xecs::{System, World, Stage};
use xecs::resource::Resource;
use std::cell::{RefMut, Ref};
use elikar::window::Window;
use elikar::events::PollEvents;
use elikar::keyboard::Code;

struct CreateWindowSystem;
impl<'a> System<'a> for CreateWindowSystem {
    type Resource = &'a mut World;
    type Dependencies = ();

    fn update(&'a mut self, mut world : RefMut<'a,World>) {
        world.register::<Window>();

        world.create_entity()
            .attach(elikar::window::Builder::default()
                .title("elikar test")
                .build()
                .unwrap());


    }
}

struct PrintSystem(String);
impl<'a> System<'a> for PrintSystem{
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self, _ : ()) {
        println!("PrintSystem:{}",&self.0);
    }
}

struct ChangeToDefaultStage;
impl<'a> System<'a> for ChangeToDefaultStage{
    type Resource = &'a mut ElikarStates;
    type Dependencies = ();

    fn update(&'a mut self, mut states : RefMut<'a,ElikarStates>) {
        states.change_current("default");
    }
}

struct CreateAStage;
impl<'a> System<'a> for CreateAStage {
    type Resource = &'a mut ElikarStates;
    type Dependencies = ();

    fn update(&'a mut self, mut states : RefMut<'a,ElikarStates>) {
        let mut stage = Stage::new();
        stage.add_system(PrintSystem("Fuck stage".to_owned()))
            .add_system(ChangeToDefaultStage);
        states.add_stage("fuck",stage);
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

struct ChangeStage;
impl<'a> System<'a> for ChangeStage {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self,(events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        if let Some(info) = events.key_down {
            if info.code == Code::P {
                states.change_current("fuck")
            }
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    game.current_stage_mut()
        .add_once_system(CreateWindowSystem)
        .add_once_system(CreateAStage)
        .add_system(PollEvents::new())
        .add_system(QuitSystem)
        .add_system(ChangeStage);

    game.run();
}
