use elikar::{Elikar, States, common::Spawner};
use futures::StreamExt;
use xecs::system::System;

fn main() {
    let mut game = Elikar::new().unwrap();

    game.window_builder()
        .build()
        .unwrap();

    let events = game.events(); 
    game.spawn_local(async move{
        let mut quit = events.on_quit();
        let world = quit.world();
        if let Some(_) = quit.next().await {
            let world = world.read().unwrap();
            let mut states = world.resource_mut::<States>().unwrap();
            states.quit();
        }
    });

    let events = game.events(); 
    game.spawn_local(async move{
        let mut mouse_down = events.on_mouse_down();
        let world = mouse_down.world();
        while let Some(button) = mouse_down.next().await {
            println!("button down:{:?}",button);
            let world = world.read().unwrap();
            let states = world.resource_ref::<States>().unwrap();
            println!("fps:{},actual_fps:{}",states.fps(),states.actual_fps());
        }
    });
    let events = game.events(); 
    game.spawn_local(async move{
        let mut motion = events.on_mouse_motion();
        while let Some(motion) = motion.next().await {
            println!("motion: {:?}",motion);
        }
    });
    let events = game.events();
    game.spawn_local(async move {
        let mut wheel = events.on_mouse_wheel();
        while let Some(wheel) = wheel.next().await {
            println!("wheel: {:?}",wheel);
        }
    });
    
    game.run();
}
