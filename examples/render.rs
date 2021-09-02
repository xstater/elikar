use elikar::{Elikar, ElikarStates};
use xecs::System;
use xecs::resource::Resource;
use elikar::events::PollEvents;
use elikar::window;
use elikar::window::Window;
use elikar::render::gl::GLContext;
use elikar::render::renderer2d::{ClearScreen, Renderer2d};

struct Quit;
impl<'a> System<'a> for Quit {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,mut states) : <Self::Resource as Resource<'a>>::Type) {
        if let Some(_) = events.quit {
            states.quit();
        }
    }
}
struct PrintFps;
impl<'a> System<'a> for PrintFps {
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,states) : <Self::Resource as Resource<'a>>::Type) {
        if let Some(_) = events.mouse_motion {
            println!("fps:{}",states.fps());
        }
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();

    let mut window = window::Builder::default()
        .title("test")
        .opengl()
        .build()
        .unwrap();
    let _gl = GLContext::from_window(&mut window).unwrap();

    game.current_stage_mut()
        .world_mut()
        .register::<Window>()
        .create_entity()
        .attach(window);

    game.current_stage_mut()
        .add_system(Quit)
        .add_system(PollEvents::new())
        .add_system(ClearScreen::black())
        .add_system(Renderer2d)
        .add_system(PrintFps);

    game.run()
}