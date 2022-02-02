use elikar::{Elikar, States, common::Spawner};
use futures::StreamExt;
use xecs::system::System;

fn main() {
    let mut game = Elikar::new().unwrap();

    let _window_id = game.window_builder()
        .resizable()
        .always_on_top()
        .skip_taskbar()
        .title("window event test")
        .build()
        .unwrap();

    let events = game.events();
    game.spawn(async move{
        let mut quit = events.on_quit();
        let world = quit.world();
        if let Some(_) = quit.next().await {
            println!("Quit");
            let world = world.read().unwrap();
            let mut states = world.resource_mut::<States>().unwrap();
            states.quit();
        }
    });

    let events = game.events();
    game.spawn(async move {
        let mut window_events = events.on_window_events();
        while let Some(event) = window_events.next().await {
            println!("{:?}",event);
        }
    });

    game.run();
}
