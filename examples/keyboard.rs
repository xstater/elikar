use elikar::{Elikar, States, common::Spawner, keyboard::Code};
use futures::StreamExt;
use xecs::system::System;

fn main() {
    let mut game = Elikar::new().unwrap();

    game.window_builder().build().unwrap();
    
    let events = game.events();
    game.spawn(async move {
        let mut key_down = events.on_key_down();
        let world = key_down.world();
        while let Some(key) = key_down.next().await {
            println!("{:?}",key);
            if key.code == Code::Escape {
                let world = world.read();
                world.resource_write::<States>().unwrap().quit();
                break;
            }
        }
    });

    game.run();
}
