use elikar::{Elikar, States, common::Spawner};
use futures::StreamExt;
use xecs::system::System;


fn main() {
    let mut game = Elikar::new().unwrap();

    game.window_builder().build().unwrap();

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
    game.spawn_local(async move {
        let mut enter_frame = events.on_enter_frame();
        while let Some(index) = enter_frame.next().await {
            println!("Enter Frame:{}",index);
        }
    });

    let events = game.events();
    game.spawn_local(async move {
        let mut update = events.on_update();
        while let Some(_) = update.next().await {
            println!("Update");
        }
    });

    let events = game.events();
    game.spawn_local(async move {
        let mut render = events.on_render();
        while let Some(_) = render.next().await {
            println!("Render");
        }
    });

    let events = game.events();
    game.spawn_local(async move {
        let mut leave_frame = events.on_leave_frame();
        while let Some(index) = leave_frame.next().await {
            println!("Leave Frame:{}",index);
        }
    });

    game.run();

}
