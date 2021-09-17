use elikar::{Elikar, ElikarStates};
use xecs::System;
use elikar::events::PollEvents;
use elikar::render::vulkan::Vulkan;
use std::sync::Arc;
use elikar::render::renderer::Renderer;
use std::cell::{Ref, RefMut};
use elikar::render::transform::Transform;
use elikar::render::sprite::Sprite;

struct Quit;
impl<'a> System<'a> for Quit {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        if let Some(_) = events.quit {
            states.quit();
        }
    }
}
struct PrintFps;
impl<'a> System<'a> for PrintFps {
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,states) : (Ref<'a,PollEvents>,Ref<'a,ElikarStates>)) {
        for _ in &events.mouse_motion {
            println!("fps:{}",states.fps());
        }
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();

    let mut manager = game.create_window_manager();
    let window_id = manager.create_window()
        .title("test")
        .vulkan()
        .build()
        .unwrap()
        .id();

    let vk = Vulkan::builder()
        .enable_debug()
        .debug_info()
        .debug_error()
        .debug_warning()
        .build(manager.window_ref(window_id).unwrap())
        .unwrap();

    let vk = Arc::new(vk);
    let renderer = Renderer::new(vk).unwrap();

    game.current_stage_mut().world_mut()
        .register::<Transform>()
        .register::<Sprite>();

    game.current_stage_mut().world_mut()
        .create_entity()
        .attach(Transform::new())
        .attach(Sprite::new());

    game.current_stage_mut()
        .add_system(Quit)
        .add_system(PollEvents::new())
        .add_system(PrintFps)
        .add_system(renderer);

    game.run()
}