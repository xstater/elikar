use elikar::{Elikar, ElikarStates, window};
use xecs::{System, Stage};
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;
use elikar::keyboard::Code;
use std::convert::Infallible;

struct PrintSystem(String);
impl<'a> System<'a> for PrintSystem{
    type InitResource = ();
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;

    fn update(&'a mut self, _ : ()) -> Result<(),Infallible> {
        println!("PrintSystem:{}",&self.0);
        Ok(())
    }
}

struct ChangeToDefaultStage;
impl<'a> System<'a> for ChangeToDefaultStage{
    type InitResource = ();
    type Resource = &'a mut ElikarStates;
    type Dependencies = ();
    type Error = Infallible;

    fn update(&'a mut self, mut states : RefMut<'a,ElikarStates>) -> Result<(),Self::Error>{
        states.change_current("default");
        Ok(())
    }
}

struct CreateAStage;
impl<'a> System<'a> for CreateAStage {
    type InitResource = &'a mut ElikarStates;
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;

    fn init(&'a mut self, mut states : RefMut<'a,ElikarStates>) -> Result<(),Infallible>{
        let mut stage = Stage::new();
        stage.add_system(PrintSystem("Fuck stage".to_owned()))
            .add_system(ChangeToDefaultStage);
        states.add_stage("fuck",stage);
        Ok(())
    }
}

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) -> Result<(),Infallible> {
        if let Some(_) = events.quit {
            states.quit()
        }
        Ok(())
    }
}

struct ChangeStage;
impl<'a> System<'a> for ChangeStage {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self,(events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) -> Result<(),Infallible>{
        for info in &events.key_down {
            if info.code == Code::P {
                states.change_current("fuck")
            }
        }
        Ok(())
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();
    {
        let mut manager = game.current_stage_ref()
            .system_data_mut::<window::Manager>();
        manager.create_window().build().unwrap();
    }
    game.current_stage_mut()
        .add_system(CreateAStage)
        .add_system(QuitSystem)
        .add_system(ChangeStage);

    game.run();
}
